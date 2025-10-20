import * as lib from "../../../src/lib";

export function getStorageInMemory(): lib.storage.Storage {
  const store = new Map<string, Uint8Array>();

  return {
    async get(key: string): Promise<Uint8Array | null> {
      return store.get(key) ?? null;
    },

    async set(key: string, value: Uint8Array): Promise<boolean> {
      store.set(key, value);
      return true;
    },

    async delete(key: string): Promise<boolean> {
      return store.delete(key);
    },

    async list(): Promise<string[]> {
      return Array.from(store.keys());
    },

    async clear(): Promise<boolean> {
      store.clear();
      return true;
    },
  };
}
