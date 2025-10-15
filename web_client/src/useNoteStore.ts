import { useSyncExternalStore } from "react";
import { useNoteStoreContext } from "./NoteStoreContext";

export function useNoteStore() {
  const store = useNoteStoreContext();

  const notes = useSyncExternalStore(store.subscribe, store.getSnapshot);

  return {
    notes,
    create: (content: string) => store.create(content),
    delete: (id: string) => store.delete(id),
  };
}
