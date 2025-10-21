export { NoteService } from "./NoteService";
export { NoteStore } from "./NoteStore";
export * as wasmNote from "./wasmNote";
export * as network from "./networks";

export type Note = {
  id: string;
  content: string;
};
