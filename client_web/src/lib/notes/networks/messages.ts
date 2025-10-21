export interface Message {
  type: string;
}

export interface Note extends Message {
  type: "note";
  id: string;
  bytes: Uint8Array;
}

export function note(id: string, bytes: Uint8Array): Note {
  return {
    type: "note",
    id,
    bytes,
  };
}

export interface Notes extends Message {
  type: "notes";
  notes: {
    id: string;
    bytes: Uint8Array;
  }[];
}

export function notes(
  notes: {
    id: string;
    bytes: Uint8Array;
  }[],
): Notes {
  return {
    type: "notes",
    notes,
  };
}

export interface Delete extends Message {
  type: "delete";
  id: string;
}

export function delete_(id: string): Delete {
  return {
    type: "delete",
    id,
  };
}
