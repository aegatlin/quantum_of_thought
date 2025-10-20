// client_web/src/lib/network/http-adapter.ts

import type { NetworkAdapter } from "./adapter";

export class HttpAdapter implements NetworkAdapter {
  readonly name = "http";
  private baseUrl: string;

  constructor(baseUrl: string = "http://localhost:4000") {
    this.baseUrl = baseUrl;
  }

  async send(id: string, data: Uint8Array): Promise<unknown> {
    const base64 = btoa(String.fromCharCode(...data));
    const response = await fetch(`${this.baseUrl}/api/notes/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ data: base64 }),
    });
    return response.json();
  }

  async get(id: string): Promise<{ id: string; data: Uint8Array } | null> {
    const response = await fetch(`${this.baseUrl}/api/notes/${id}`);
    if (!response.ok) return null;
    const json = await response.json();
    const data = Uint8Array.from(atob(json.data), (c) => c.charCodeAt(0));
    return { id: json.id, data };
  }

  async getAll(): Promise<Array<{ id: string; data: Uint8Array }>> {
    const response = await fetch(`${this.baseUrl}/api/notes`);
    const json = await response.json();
    type JsonNote = { id: string; data: string };
    return json.notes.map((note: JsonNote) => ({
      id: note.id,
      data: Uint8Array.from(atob(note.data), (c) => c.charCodeAt(0)),
    }));
  }

  async delete(id: string): Promise<unknown> {
    const response = await fetch(`${this.baseUrl}/api/notes/${id}`, {
      method: "DELETE",
    });
    return response.json();
  }
}
