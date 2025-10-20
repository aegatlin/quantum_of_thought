import * as lib from "@/lib";
import { Button } from "./ui/button";
import { Card, CardContent } from "./ui/card";

interface NoteCardProps {
  note: lib.notes.Note;
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
