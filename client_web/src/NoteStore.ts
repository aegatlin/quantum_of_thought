import { WasmNote } from "crdt_note";

export interface Note {
  id: string;
  content: string;
}

export class NoteStore {
  private notes: Map<string, WasmNote> = new Map();
  private listeners = new Set<() => void>();
  private cachedSnapshot: Note[] = [];

  private constructor() {
    this.updateSnapshot();
  }

  static async create(): Promise<NoteStore> {
    return new NoteStore();
  }

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  getSnapshot = (): Note[] => {
    return this.cachedSnapshot;
  };

  create(content: string): Note {
    const wasmNote = new WasmNote(content);
    const id = wasmNote.id();
    const noteContent = wasmNote.content();

    this.notes.set(id, wasmNote);
    this.updateSnapshot();
    this.notifyListeners();

    return { id, content: noteContent };
  }

  delete(id: string): boolean {
    try {
      const deleted = this.notes.delete(id);
      if (deleted) {
        this.updateSnapshot();
        this.notifyListeners();
      }
      return deleted;
    } catch (error) {
      console.error(`Failed to delete note ${id}:`, error);
      return false;
    }
  }

  private updateSnapshot(): void {
    this.cachedSnapshot = Array.from(this.notes.values()).map((wasmNote) => ({
      id: wasmNote.id(),
      content: wasmNote.content(),
    }));
  }

  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener());
  }
}
