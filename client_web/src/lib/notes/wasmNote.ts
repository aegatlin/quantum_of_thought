import * as lib from "@/lib";
import * as wasm from "crdt_note";

export type WasmNote = wasm.Note;

export function wnote_from_content(content: string): WasmNote {
  return wasm.Note.new(content);
}

export function wnote_from_bytes(bytes: Uint8Array): WasmNote {
  return wasm.Note.from(bytes);
}

export function wnote_into_note(wasmNote: WasmNote): lib.notes.Note {
  return {
    id: wasmNote.id(),
    content: wasmNote.content(),
  };
}
