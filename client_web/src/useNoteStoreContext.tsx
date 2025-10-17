import { useContext } from "react";
import type { NoteStore } from "./NoteStore";
import { NoteStoreContext } from "./NoteStoreContext";

export function useNoteStoreContext(): NoteStore {
  const context = useContext(NoteStoreContext);
  if (!context) {
    throw new Error(
      "useNoteStoreContext must be used within NoteStoreProvider",
    );
  }
  return context.store;
}
