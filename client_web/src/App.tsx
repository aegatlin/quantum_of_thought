import { NoteStoreProvider } from "./NoteStoreContext";
import { useNoteStore } from "./useNoteStore";
import { Button } from "./components/ui/button";
import { Card, CardContent } from "./components/ui/card";
import { NoteCard } from "./components/NoteCard";

function App() {
  return (
    <NoteStoreProvider>
      <NotesApp />
    </NoteStoreProvider>
  );
}

function NotesApp() {
  const noteStore = useNoteStore();

  return (
    <div className="min-h-screen bg-background p-8">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-4xl font-bold mb-8">Quantum of Thought</h1>

        <Button onClick={() => noteStore.create("New note!")} className="mb-6">
          Add Note
        </Button>

        {noteStore.notes.length === 0 ? (
          <Card>
            <CardContent className="flex items-center justify-center p-12">
              <p className="text-muted-foreground">
                No notes yet. Create your first note!
              </p>
            </CardContent>
          </Card>
        ) : (
          <div className="space-y-3">
            {noteStore.notes.map((note) => (
              <NoteCard key={note.id} note={note} onDelete={noteStore.delete} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
