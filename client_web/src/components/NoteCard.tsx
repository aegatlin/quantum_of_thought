import { useState } from "react";
import * as lib from "@/lib";
import { Button } from "./ui/button";
import { Card, CardContent } from "./ui/card";
import { NoteEditDialog } from "./NoteEditDialog";

interface NoteCardProps {
  note: lib.notes.Note;
  onDelete: (id: string) => void;
  onEdit: (id: string, content: string) => void;
}

export function NoteCard({ note, onDelete, onEdit }: NoteCardProps) {
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false);

  return (
    <>
      <Card
        className="cursor-pointer hover:bg-accent transition-colors"
        onClick={() => setIsEditDialogOpen(true)}
      >
        <CardContent className="flex justify-between items-center p-4">
          <span className="flex-1">{note.content}</span>
          <Button
            variant="destructive"
            size="sm"
            onClick={(e) => {
              e.stopPropagation(); // Prevent card click from triggering
              onDelete(note.id);
            }}
          >
            Delete
          </Button>
        </CardContent>
      </Card>

      <NoteEditDialog
        note={note}
        isOpen={isEditDialogOpen}
        onClose={() => setIsEditDialogOpen(false)}
        onSave={onEdit}
      />
    </>
  );
}
