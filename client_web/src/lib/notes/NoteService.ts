import * as lib from "@/lib";
import * as wasm from "crdt_note";

/*
Add listeners via the subscribe() function to receive updates.

All functions are synchronous, returning appropriate data immediate. Any data
that requires asynchronous activity will notify listeners when they are
complete.
*/
export class NoteService {
  private wnotes: Map<string, wasm.Note> = new Map();
  private storage: lib.storage.Storage = lib.storage.getStorage();
  private listeners = new Set<() => void>();

  constructor() {
    this.update();
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  all(): lib.notes.Note[] {
    this.update();
    return Array.from(this.wnotes.values()).map((wnote) => this.view(wnote));
  }

  create(content: string): lib.notes.Note {
    const wnote = wasm.Note.new(content);
    const note = this.view(wnote);
    this.wnotes.set(note.id, wnote);

    const wnoteData = wnote.into();
    this.storage.set(note.id, wnoteData).then(() => {
      // no-op
    });

    return note;
  }

  get(id: string): lib.notes.Note | null {
    // get from in-memory
    if (this.wnotes.has(id)) {
      const wnote = this.wnotes.get(id);
      return wnote ? this.view(wnote) : null;
    }

    // async get from storage, then set in memory
    this.storage.get(id).then((wnoteData) => {
      if (wnoteData) {
        const wnote = wasm.Note.from(wnoteData);
        const note = this.view(wnote);
        this.wnotes.set(note.id, wnote);
        this.notify();
      }
    });

    return null;
  }

  delete(id: string): boolean {
    // delete from memory
    const isDeletedFromMemory = this.wnotes.delete(id);

    // async delete from storage
    this.storage.delete(id).then(() => {
      this.notify();
    });

    return isDeletedFromMemory;
  }

  private notify() {
    this.listeners.forEach((listener) => listener());
  }

  /**
   * Update should only call `this.notify()` if an actual change occurs in `this.wnotes`
   */
  private update() {
    this.storage.list().then((ids) => {
      ids.forEach((id) => {
        if (!this.wnotes.has(id)) {
          this.storage.get(id).then((wnoteData) => {
            if (wnoteData) {
              const wnote = wasm.Note.from(wnoteData);
              const note = this.view(wnote);
              this.wnotes.set(note.id, wnote);
            }

            this.notify();
          });
        }
      });
    });
  }

  private view(note: wasm.Note): lib.notes.Note {
    return {
      id: note.id(),
      content: note.content(),
    };
  }
}
