import localforage from "localforage";

export interface Storage {
  get(key: string): Promise<Uint8Array | null>;
  set(key: string, value: Uint8Array): Promise<boolean>;
  delete(key: string): Promise<boolean>;
  list(): Promise<string[]>;
  clear(): Promise<boolean>;
}

export function getStorage(): Storage {
  return {
    async get(key) {
      return await localforage.getItem<Uint8Array>(key);
    },

    async set(key, value) {
      try {
        await localforage.setItem(key, value);
        return true;
      } catch {
        return false;
      }
    },

    async delete(key) {
      try {
        await localforage.removeItem(key);
        return true;
      } catch {
        return false;
      }
    },

    async list() {
      return await localforage.keys();
    },

    async clear() {
      try {
        await localforage.clear();
        return true;
      } catch {
        return false;
      }
    },
  };
}
