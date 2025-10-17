import { createContext } from "react";
import type { NoteStore } from "./NoteStore";

export interface NoteStoreContextValue {
  store: NoteStore;
}

export const NoteStoreContext = createContext<NoteStoreContextValue | null>(
  null,
);
