export { NoteService } from "./NoteService";
export { NoteStore } from "./NoteStore";
export * as wasmNote from "./wasmNote";

export type Note = {
  id: string;
  content: string;
};
