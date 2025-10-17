import * as lib from "@/lib";

export class NoteStore {
  private notes: Map<string, lib.note.Note> = new Map();
  private listeners = new Set<() => void>();
  private cachedSnapshot: lib.note.Note[] = [];

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

  getSnapshot = (): lib.note.Note[] => {
    return this.cachedSnapshot;
  };

  create(content: string): lib.note.Note {
    const note = lib.note.new_note(content);
    this.notes.set(note.id, note);
    this.updateSnapshot();
    this.notifyListeners();
    return note;
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
    this.cachedSnapshot = Array.from(this.notes.values());
  }

  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener());
  }
}
