import { useSyncExternalStore } from "react";
import { useNoteStoreContext } from "./useNoteStoreContext";

export function useNoteStore() {
  const store = useNoteStoreContext();

  const notes = useSyncExternalStore(
    (listener) => store.subscribe(listener),
    () => {
      return store.getSnapshot();
    },
  );

  return {
    notes,
    create: (content: string) => store.create(content),
    delete: (id: string) => store.delete(id),
  };
}
