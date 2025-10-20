export interface NetworkAdapter {
  readonly name: string;
  send(id: string, data: Uint8Array): Promise<unknown>;
  get(id: string): Promise<{ id: string; data: Uint8Array } | null>;
  getAll(): Promise<Array<{ id: string; data: Uint8Array }>>;
  delete(id: string): Promise<unknown>;
}
