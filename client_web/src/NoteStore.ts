import init, { WasmNotes } from "crdt_notes";

export interface Note {
  id: string;
  content: string;
}

export class NoteStore {
  private wasm: WasmNotes;
  private listeners = new Set<() => void>();
  private cachedSnapshot: Note[] = [];

  private constructor(wasm: WasmNotes) {
    this.wasm = wasm;
    this.updateSnapshot();
  }

  static async create(): Promise<NoteStore> {
    await init();
    const wasm = new WasmNotes();
    return new NoteStore(wasm);
  }

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  getSnapshot = (): Note[] => {
    return this.cachedSnapshot;
  };

  create(content: string): Note {
    const note = this.wasm.create(content);
    this.updateSnapshot();
    this.notifyListeners();
    return note;
  }

  delete(id: string): boolean {
    try {
      this.wasm.delete(id);
      this.updateSnapshot();
      this.notifyListeners();
      return true;
    } catch (error) {
      console.error(`Failed to delete note ${id}:`, error);
      return false;
    }
  }

  private updateSnapshot(): void {
    this.cachedSnapshot = this.wasm.list();
  }

  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener());
  }
}
