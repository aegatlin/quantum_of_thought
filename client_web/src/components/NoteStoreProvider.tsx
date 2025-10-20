import * as lib from "@/lib";
import type { ReactNode } from "react";
import { useState } from "react";
import { NoteStoreContext } from "./NoteStoreContext";

export function NoteStoreProvider({ children }: { children: ReactNode }) {
  const [store] = useState<lib.notes.NoteStore>(new lib.notes.NoteStore());

  return (
    <NoteStoreContext.Provider value={{ store }}>
      {children}
    </NoteStoreContext.Provider>
  );
}
