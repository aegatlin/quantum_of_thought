import { Card, CardContent } from "./ui/card";
import { Button } from "./ui/button";
import type { Note } from "../NoteStore";

interface NoteCardProps {
  note: Note;
  onDelete: (id: string) => void;
}

export function NoteCard({ note, onDelete }: NoteCardProps) {
  return (
    <Card>
      <CardContent className="flex justify-between items-center p-4">
        <span className="flex-1">{note.content}</span>
        <Button
          variant="destructive"
          size="sm"
          onClick={() => onDelete(note.id)}
        >
          Delete
        </Button>
      </CardContent>
    </Card>
  );
}
