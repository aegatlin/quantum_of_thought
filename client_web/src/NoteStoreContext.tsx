import * as lib from "@/lib";
import { createContext } from "react";

export interface NoteStoreContextValue {
  store: lib.notes.NoteStore;
}

export const NoteStoreContext = createContext<NoteStoreContextValue | null>(
  null,
);
