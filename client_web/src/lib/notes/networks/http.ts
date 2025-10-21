import type { Network } from "./index";
import * as messages from "./messages";

type Message = messages.Message;

export class Http implements Network {
  #baseUrl: string;
  #listeners = new Set<(message: Message) => void>();

  constructor(baseUrl: string = "http://localhost:4000") {
    this.#baseUrl = baseUrl;
  }

  send(message: Message): void {
    switch (message.type) {
      case "note":
        this.#sendNote(message as messages.Note);
        break;
      case "delete":
        this.#deleteNote(message as messages.Delete);
        break;
    }

    this.#fetchAllNotesAndNotify();
  }

  subscribe(listener: (message: Message) => void): () => void {
    this.#listeners.add(listener);

    return () => {
      this.#listeners.delete(listener);
    };
  }

  async #sendNote(note: messages.Note): Promise<void> {
    try {
      const base64 = btoa(String.fromCharCode(...note.bytes));

      await fetch(`${this.#baseUrl}/api/notes/${note.id}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: note.id, data: base64 }),
      });
    } catch (err) {
      console.error("[http] send error:", err);
    }
  }

  async #deleteNote(deleteMsg: messages.Delete): Promise<void> {
    try {
      await fetch(`${this.#baseUrl}/api/notes/${deleteMsg.id}`, {
        method: "DELETE",
      });
    } catch (err) {
      console.error("[http] delete error:", err);
    }
  }

  async #fetchAllNotesAndNotify(): Promise<void> {
    try {
      const response = await fetch(`${this.#baseUrl}/api/notes`);
      const json = await response.json();

      const notesList = json.notes.map(
        (note: { id: string; data: string }) => ({
          id: note.id,
          bytes: Uint8Array.from(atob(note.data), (c) => c.charCodeAt(0)),
        }),
      );

      const notesMessage = messages.notes(notesList);

      this.#listeners.forEach((listener) => {
        try {
          listener(notesMessage);
        } catch (err) {
          console.error("[http] listener error:", err);
        }
      });
    } catch (err) {
      console.error("[http] fetch error:", err);
    }
  }

  disconnect(): void {
    this.#listeners.clear();
  }
}
