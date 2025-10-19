import { describe, it, expect } from "vitest";
import * as lib from "@/lib";

describe("new_note", () => {
  it("creates a note with content", () => {
    const note = lib.note.new_note("Hello world");

    expect(note.content).toBe("Hello world");
    expect(note.id).toBeDefined();
    expect(typeof note.id).toBe("string");
  });

  it("creates unique IDs for different notes", () => {
    const note1 = lib.note.new_note("First");
    const note2 = lib.note.new_note("Second");

    expect(note1.id).not.toBe(note2.id);
  });
});
