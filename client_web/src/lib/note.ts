import * as wasm from "crdt_note";

export type Note = {
  id: string;
  content: string;
};

export function new_note(content: string): Note {
  const n = wasm.Note.new(content);

  return {
    id: n.id(),
    content: n.content(),
  };
}
