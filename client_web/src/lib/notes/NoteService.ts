import * as lib from "@/lib";

interface INoteService {
  subscribe(listener: () => void): () => void;

  list(): lib.notes.Note[];

  create(content: string): lib.notes.Note;
  get(id: string): lib.notes.Note | null;
  update(id: string, content: string): lib.notes.Note | null;
  delete(id: string): void;
}

/**
 *
 * Add listeners via the subscribe() function to receive updates.
 *
 * All public API functions are synchronous. Any data that requires asynchronous
 * activity will notify listeners when they are complete.
 *
 */
export class NoteService implements INoteService {
  #storage: lib.storage.Storage;
  #networks: lib.notes.network.Network[];

  #wnotes: Map<string, lib.notes.wasmNote.WasmNote> = new Map();
  #listeners = new Set<() => void>();

  constructor(opts = { storage: lib.storage.getStorage() }) {
    this.#storage = opts.storage;
    this.#networks = [
      new lib.notes.network.Http(),
      new lib.notes.network.WebSocket(),
    ];

    // Subscribe to incoming network messages
    this.#networks.forEach((network) => {
      network.subscribe((message) => {
        this.#handleNetworkMessage(message);
      });
    });

    // The in-memory list should be empty but this will also kick of the action
    // of getting the list from storage.
    this.list();
  }

  subscribe(listener: () => void): () => void {
    this.#listeners.add(listener);
    return () => this.#listeners.delete(listener);
  }

  #notify() {
    this.#listeners.forEach((listener) => listener());
  }

  #handleNetworkMessage(message: lib.notes.network.messages.Message): void {
    switch (message.type) {
      case "notes": {
        const notesMsg = message as lib.notes.network.messages.Notes;

        let hasChanges = false;

        notesMsg.notes.forEach(({ id, bytes }) => {
          if (!this.#wnotes.has(id)) {
            hasChanges = true;
            const wnote = lib.notes.wasmNote.wnote_from_bytes(bytes);
            this.#wnotes.set(id, wnote);
            // Also save to storage
            this.#storage.set(id, bytes);
          }
        });

        if (hasChanges) {
          this.#notify();
        }

        break;
      }
      case "note": {
        const noteMsg = message as lib.notes.network.messages.Note;

        if (!this.#wnotes.has(noteMsg.id)) {
          const wnote = lib.notes.wasmNote.wnote_from_bytes(noteMsg.bytes);
          this.#wnotes.set(noteMsg.id, wnote);
          this.#storage.set(noteMsg.id, noteMsg.bytes);
          this.#notify();
        }

        break;
      }
      // should the delete message be retransmitted across networks? I assume
      // yes because it should not be assumed all networks have been informed,
      // and handling the redundancy of the message should be preferred (so long
      // as there is no infinite loop).
      case "delete": {
        const deleteMsg = message as lib.notes.network.messages.Delete;

        const wasDeleted = this.#wnotes.delete(deleteMsg.id);

        if (wasDeleted) {
          // Also delete from storage
          this.#storage.delete(deleteMsg.id);
          this.#notify();
        }

        break;
      }
    }
  }

  list(): lib.notes.Note[] {
    this.#storage.list().then((ids) => {
      ids.forEach((id) => {
        if (!this.#wnotes.has(id)) {
          this.#storage.get(id).then((wnoteData) => {
            if (wnoteData) {
              const wnote = lib.notes.wasmNote.wnote_from_bytes(wnoteData);
              const note = lib.notes.wasmNote.wnote_into_note(wnote);
              this.#wnotes.set(note.id, wnote);
            }

            this.#notify();
          });
        }
      });
    });

    return Array.from(this.#wnotes.values()).map((wnote) =>
      lib.notes.wasmNote.wnote_into_note(wnote),
    );
  }

  create(content: string): lib.notes.Note {
    const wnote = lib.notes.wasmNote.wnote_from_content(content);
    const note = lib.notes.wasmNote.wnote_into_note(wnote);
    this.#wnotes.set(note.id, wnote);

    const wnoteData = wnote.into();

    // storage
    this.#storage.set(note.id, wnoteData);

    // networks
    const noteMessage = lib.notes.network.messages.note(note.id, wnoteData);
    this.#networks.forEach((network) => {
      network.send(noteMessage);
    });

    return note;
  }

  get(id: string): lib.notes.Note | null {
    // in memory
    if (this.#wnotes.has(id)) {
      const wnote = this.#wnotes.get(id);
      return wnote ? lib.notes.wasmNote.wnote_into_note(wnote) : null;
    }

    // storag
    this.#storage.get(id).then((wnoteData) => {
      if (wnoteData) {
        const wnote = lib.notes.wasmNote.wnote_from_bytes(wnoteData);
        const note = lib.notes.wasmNote.wnote_into_note(wnote);
        this.#wnotes.set(note.id, wnote);
        this.#notify();
      }
    });

    return null;
  }

  update(id: string, content: string): lib.notes.Note | null {
    if (this.#wnotes.has(id)) {
      const wnote = this.#wnotes.get(id);

      if (wnote) {
        const newWnote = wnote.update(content);
        // overwrites the old wnote, which should have the same id.
        this.#wnotes.set(newWnote.id(), newWnote);

        const wnoteData = newWnote.into();
        this.#storage.set(newWnote.id(), wnoteData);

        // Broadcast update to networks
        const noteMessage = lib.notes.network.messages.note(
          newWnote.id(),
          wnoteData,
        );
        this.#networks.forEach((network) => {
          network.send(noteMessage);
        });

        return lib.notes.wasmNote.wnote_into_note(newWnote);
      }
    }

    return null;
  }

  delete(id: string): void {
    // memory
    this.#wnotes.delete(id);

    // storage
    this.#storage.delete(id).then(() => {
      this.#notify();
    });

    // networks
    const deleteMessage = lib.notes.network.messages.delete_(id);
    this.#networks.forEach((network) => {
      network.send(deleteMessage);
    });
  }
}
