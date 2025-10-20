import * as wasm from "crdt_note";

export function wnote(): wasm.Note {
  return wasm.Note.new("wow");
}

export function noteBytesFrom(note: wasm.Note): Uint8Array {
  return note.into();
}

export function noteFrom(bytes: Uint8Array): wasm.Note {
  return wasm.Note.from(bytes);
}
