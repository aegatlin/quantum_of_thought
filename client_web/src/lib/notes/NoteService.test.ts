import { describe, expect, it } from "vitest";
import * as tt from "../../../tests/tool";
import { NoteService } from "./NoteService";

async function setup() {
  const storage = tt.storage.getStorageInMemory();
  const wnote = tt.fake.wnote();

  storage.set(wnote.id(), wnote.into());

  const noteService = new NoteService({ storage });

  let notifyCount = 0;
  noteService.subscribe(() => {
    notifyCount += 1;
  });

  return {
    storage,
    wnote,
    notifyCount,
    noteService: new NoteService({ storage }),
    untilNotifyGte(n: number) {
      return new Promise<void>((resolve) => {
        const interval = setInterval(() => {
          if (notifyCount >= n) {
            clearInterval(interval);
            resolve();
          }
        }, 10);
      });
    },
    waitForNotify: async () => {
      if (notifyCount < 1) {
        await new Promise((resolve) => setTimeout(resolve, 100));
      }
    },
  };
}

describe("NoteService", () => {
  it("create: should create a note with content", async () => {
    const { noteService } = await setup();
    const expected = "Test note content";

    const note = noteService.create(expected);

    expect(note).toBeDefined();
    expect(note.id).toBeTruthy();
    expect(note.content).toBe(expected);
  });

  it("all", async () => {
    const { noteService, wnote, untilNotifyGte } = await setup();
    await untilNotifyGte(1);

    const notes = noteService.all();

    expect(notes).toHaveLength(1);
    expect(notes[0].id).toBe(wnote.id());
    expect(notes[0].content).toBe(wnote.content());
  });

  it("delete", async () => {
    const { storage, noteService, wnote, untilNotifyGte } = await setup();
    await untilNotifyGte(1);

    const bool = noteService.delete(wnote.id());
    expect(bool).toBeTruthy();

    const bool2 = await storage.get(wnote.id());
    expect(bool2).toBeFalsy();
  });

  it("get", async () => {
    const { noteService, wnote, untilNotifyGte } = await setup();
    await untilNotifyGte(1);

    const note = noteService.get(wnote.id())!;

    expect(note).toBeDefined();
    expect(note.id).toBe(wnote.id());
    expect(note.content).toBe(wnote.content());
  });
});
