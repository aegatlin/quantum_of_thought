import { useState } from "react";
import { useNoteStore } from "./useNoteStore";
import { Button } from "./ui/button";
import { Card, CardContent } from "./ui/card";
import { Input } from "./ui/input";
import { NoteCard } from "./NoteCard";
import { NoteStoreProvider } from "./NoteStoreProvider";

function App() {
  return (
    <NoteStoreProvider>
      <NotesApp />
    </NoteStoreProvider>
  );
}

function NotesApp() {
  const noteStore = useNoteStore();
  const [noteText, setNoteText] = useState("");

  const handleAddNote = () => {
    if (noteText.trim()) {
      noteStore.create(noteText);
      setNoteText("");
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleAddNote();
    }
  };

  const handleEditNote = (id: string, content: string) => {
    noteStore.update(id, content);
  };

  return (
    <div className="min-h-screen bg-background p-8">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-4xl font-bold mb-8">Quantum of Thought</h1>

        <div className="flex gap-2 mb-6">
          <Input
            type="text"
            placeholder="Enter note text..."
            value={noteText}
            onChange={(e) => setNoteText(e.target.value)}
            onKeyPress={handleKeyPress}
            className="flex-1"
          />
          <Button onClick={handleAddNote}>Add Note</Button>
        </div>

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
              <NoteCard
                key={note.id}
                note={note}
                onDelete={noteStore.delete}
                onEdit={handleEditNote}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
