import * as lib from "@/lib";

// React-sensitive "external store" to be used with React.useSyncExternalStore
//
// https://react.dev/reference/react/useSyncExternalStore
//
export class NoteStore {
  private listeners = new Set<() => void>();
  private cachedSnapshot: lib.notes.Note[] = [];

  private noteService: lib.notes.NoteService;

  constructor() {
    this.noteService = new lib.notes.NoteService();
    this.noteService.subscribe(() => {
      this.updateSnapshot();
      this.notifyListeners();
    });

    this.updateSnapshot();
    this.notifyListeners();
  }

  // React API
  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  // React API
  getSnapshot = (): lib.notes.Note[] => {
    return this.cachedSnapshot;
  };

  create(content: string): lib.notes.Note {
    const note = this.noteService.create(content);
    this.updateSnapshot();
    this.notifyListeners();
    return note;
  }

  delete(id: string): boolean {
    const isDeleted = this.noteService.delete(id);
    this.updateSnapshot();
    this.notifyListeners();
    return isDeleted;
  }

  update(id: string, content: string): lib.notes.Note | null {
    const newNote = this.noteService.update(id, content);
    if (newNote) {
      this.updateSnapshot();
      this.notifyListeners();
    }
    return newNote;
  }

  private updateSnapshot(): void {
    this.cachedSnapshot = this.noteService.all();
  }

  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener());
  }
}
