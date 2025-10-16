import { createContext, useContext, useState, useEffect } from "react";
import type { ReactNode } from "react";
import { NoteStore } from "./NoteStore";

interface NoteStoreContextValue {
  store: NoteStore;
}

const NoteStoreContext = createContext<NoteStoreContextValue | null>(null);

export function NoteStoreProvider({ children }: { children: ReactNode }) {
  const [store, setStore] = useState<NoteStore | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    NoteStore.create().then((newStore) => {
      setStore(newStore);
      setIsLoading(false);
    });
  }, []);

  if (isLoading || !store) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-4xl font-bold tracking-tight">
            Quantum of Thought
          </h1>
          <p className="mt-4 text-muted-foreground">Initializing...</p>
        </div>
      </div>
    );
  }

  return (
    <NoteStoreContext.Provider value={{ store }}>
      {children}
    </NoteStoreContext.Provider>
  );
}

export function useNoteStoreContext(): NoteStore {
  const context = useContext(NoteStoreContext);
  if (!context) {
    throw new Error(
      "useNoteStoreContext must be used within NoteStoreProvider",
    );
  }
  return context.store;
}
