import * as lib from "@/lib";
import { useContext } from "react";
import { NoteStoreContext } from "./NoteStoreContext";

export function useNoteStoreContext(): lib.notes.NoteStore {
  const context = useContext(NoteStoreContext);
  if (!context) {
    throw new Error(
      "useNoteStoreContext must be used within NoteStoreProvider",
    );
  }
  return context.store;
}
