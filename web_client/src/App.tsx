import { useEffect, useState } from "react";
import { NoteStore } from "./NoteStore";
import { useNoteStore } from "./useNoteStore";

function App() {
  const [store, setStore] = useState<NoteStore | null>(null);

  useEffect(() => {
    NoteStore.create().then(setStore);
  }, []);

  if (!store) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-4xl font-bold tracking-tight">
            Quantum of Thought
          </h1>
          <p className="mt-4 text-muted-foreground">Initializing...</p>
        </div>
      </div>
    );
  }

  return <NotesApp store={store} />;
}

function NotesApp({ store }: { store: NoteStore }) {
  const noteStore = useNoteStore(store);

  return (
    <div className="min-h-screen bg-background p-8">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-4xl font-bold mb-8">Quantum of Thought</h1>

        <button
          onClick={() => noteStore.create("New note!")}
          className="mb-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Add Note
        </button>

        <ul className="space-y-2">
          {noteStore.notes.map((note) => (
            <li
              key={note.id}
              className="p-4 border rounded flex justify-between items-center"
            >
              <span>{note.content}</span>
              <button
                onClick={() => noteStore.delete(note.id)}
                className="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600"
              >
                Delete
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;
