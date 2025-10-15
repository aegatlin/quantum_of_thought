import { useSyncExternalStore } from 'react';
import type { NoteStore } from './NoteStore';

export function useNoteStore(store: NoteStore) {
  const notes = useSyncExternalStore(
    store.subscribe,
    store.getSnapshot
  );

  return {
    notes,
    create: (content: string) => store.create(content),
    delete: (id: string) => store.delete(id),
  };
}
