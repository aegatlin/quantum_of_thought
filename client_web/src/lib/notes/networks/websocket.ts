import type { Network } from "./index";
import * as messages from "./messages";

type Message = messages.Message;

export class WebSocket implements Network {
  #ws: globalThis.WebSocket | null = null;
  #url: string;
  #listeners = new Set<(message: Message) => void>();
  #reconnectInterval = 5000;
  #reconnectTimer?: number;
  #messageRef = 0;
  #joined = false;

  constructor(url: string = "ws://localhost:4000/socket/websocket") {
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

      // Join the "notes:lobby" channel
      this.#joinChannel();
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
      this.#joined = false;

      // Schedule reconnect
      this.#reconnectTimer = setTimeout(() => {
        this.#connect();
      }, this.#reconnectInterval) as unknown as number;
    };
  }

  #joinChannel(): void {
    const joinMessage = {
      topic: "notes:lobby",
      event: "phx_join",
      payload: {},
      ref: String(this.#messageRef++),
    };

    this.#ws?.send(JSON.stringify(joinMessage));
  }

  send(message: Message): void {
    if (
      !this.#ws ||
      this.#ws.readyState !== globalThis.WebSocket.OPEN ||
      !this.#joined
    ) {
      console.warn("[websocket] Cannot send, not connected or not joined");
      return;
    }

    // Convert message to match server format (base64 encoded data)
    let payload: any;

    if (message.type === "note") {
      const noteMsg = message as messages.Note;
      payload = {
        type: "note",
        id: noteMsg.id,
        data: btoa(String.fromCharCode(...noteMsg.bytes)),
      };
    } else if (message.type === "delete") {
      payload = message;
    } else {
      payload = message;
    }

    // Wrap in Phoenix Channel protocol
    const channelMessage = {
      topic: "notes:lobby",
      event: "message",
      payload: payload,
      ref: String(this.#messageRef++),
    };

    this.#ws.send(JSON.stringify(channelMessage));
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

      const channelMsg = JSON.parse(jsonString);

      // Handle Phoenix Channel events
      if (
        channelMsg.event === "phx_reply" &&
        channelMsg.payload.status === "ok"
      ) {
        // Successfully joined channel
        console.log("[websocket] Joined channel:", channelMsg.topic);
        this.#joined = true;
        return;
      }

      if (channelMsg.event === "message") {
        // Extract our application message from the channel payload
        const parsed = channelMsg.payload;

        // Reconstruct Uint8Array fields if needed
        let message: Message;

        if (parsed.type === "note") {
          // Phoenix sends base64 encoded data, need to decode
          const binaryString = atob(parsed.data);
          const bytes = new Uint8Array(binaryString.length);
          for (let i = 0; i < binaryString.length; i++) {
            bytes[i] = binaryString.charCodeAt(i);
          }
          message = messages.note(parsed.id, bytes);
        } else if (parsed.type === "notes") {
          message = messages.notes(
            parsed.notes.map((n: { id: string; data: string }) => {
              const binaryString = atob(n.data);
              const bytes = new Uint8Array(binaryString.length);
              for (let i = 0; i < binaryString.length; i++) {
                bytes[i] = binaryString.charCodeAt(i);
              }
              return {
                id: n.id,
                bytes: bytes,
              };
            }),
          );
        } else if (parsed.type === "delete") {
          message = messages.delete_(parsed.id);
        } else {
          message = parsed;
        }

        this.#notifyListeners(message);
      }
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
