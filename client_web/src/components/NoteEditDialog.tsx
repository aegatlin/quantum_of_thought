import { useState, useEffect } from "react";
import * as lib from "@/lib";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Textarea } from "./ui/textarea";
import { Button } from "./ui/button";

interface NoteEditDialogProps {
  note: lib.notes.Note;
  isOpen: boolean;
  onClose: () => void;
  onSave: (id: string, content: string) => void;
}

export function NoteEditDialog({
  note,
  isOpen,
  onClose,
  onSave,
}: NoteEditDialogProps) {
  const [editedContent, setEditedContent] = useState(note.content);

  // Reset content when note changes or dialog opens
  useEffect(() => {
    setEditedContent(note.content);
  }, [note.content, isOpen]);

  const handleSave = () => {
    onSave(note.id, editedContent);
    onClose();
  };

  const handleCancel = () => {
    setEditedContent(note.content); // Reset to original
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>Edit Note</DialogTitle>
        </DialogHeader>
        <div className="py-4">
          <Textarea
            value={editedContent}
            onChange={(e) => setEditedContent(e.target.value)}
            placeholder="Edit your note..."
            className="min-h-[200px]"
            autoFocus
          />
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={handleCancel}>
            Cancel
          </Button>
          <Button onClick={handleSave}>Save</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
