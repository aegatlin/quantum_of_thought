import type { Network } from "./index";
import * as messages from "./messages";

type Message = messages.Message;

export class WebSocket implements Network {
  #ws: globalThis.WebSocket | null = null;
  #url: string;
  #listeners = new Set<(message: Message) => void>();
  #reconnectInterval = 5000;
  #reconnectTimer?: number;

  constructor(url: string = "ws://localhost:4000/socket") {
    this.#url = url;
    this.#connect();
  }

  #connect(): void {
    this.#ws = new globalThis.WebSocket(this.#url);

    this.#ws.onopen = () => {
      console.log("[websocket] Connected");
      if (this.#reconnectTimer) {
        clearTimeout(this.#reconnectTimer);
        this.#reconnectTimer = undefined;
      }
    };

    this.#ws.onmessage = (event) => {
      this.#handleMessage(event.data);
    };

    this.#ws.onerror = (error) => {
      console.error("[websocket] Error:", error);
    };

    this.#ws.onclose = () => {
      console.log("[websocket] Disconnected, reconnecting...");
      this.#ws = null;

      // Schedule reconnect
      this.#reconnectTimer = setTimeout(() => {
        this.#connect();
      }, this.#reconnectInterval) as unknown as number;
    };
  }

  send(message: Message): void {
    if (!this.#ws || this.#ws.readyState !== globalThis.WebSocket.OPEN) {
      console.warn("[websocket] Cannot send, not connected");
      return;
    }

    // Serialize message to JSON
    const json = JSON.stringify(message);
    this.#ws.send(json);
  }

  subscribe(listener: (message: Message) => void): () => void {
    this.#listeners.add(listener);

    return () => {
      this.#listeners.delete(listener);
    };
  }

  async #handleMessage(data: string | Blob | ArrayBuffer): Promise<void> {
    try {
      let jsonString: string;

      if (data instanceof Blob) {
        jsonString = await data.text();
      } else if (data instanceof ArrayBuffer) {
        const decoder = new TextDecoder();
        jsonString = decoder.decode(data);
      } else {
        jsonString = data;
      }

      const parsed = JSON.parse(jsonString);

      // Reconstruct Uint8Array fields if needed
      let message: Message;

      if (parsed.type === "note") {
        message = messages.note(parsed.id, new Uint8Array(parsed.bytes));
      } else if (parsed.type === "notes") {
        message = messages.notes(
          parsed.notes.map((n: { id: string; bytes: number[] }) => ({
            id: n.id,
            bytes: new Uint8Array(n.bytes),
          })),
        );
      } else if (parsed.type === "delete") {
        message = messages.delete_(parsed.id);
      } else {
        message = parsed;
      }

      this.#notifyListeners(message);
    } catch (err) {
      console.error("[websocket] message parse error:", err);
    }
  }

  #notifyListeners(message: Message): void {
    this.#listeners.forEach((listener) => {
      try {
        listener(message);
      } catch (err) {
        console.error("[websocket] listener error:", err);
      }
    });
  }

  disconnect(): void {
    if (this.#reconnectTimer) {
      clearTimeout(this.#reconnectTimer);
    }
    if (this.#ws) {
      this.#ws.close();
      this.#ws = null;
    }
    this.#listeners.clear();
  }
}
