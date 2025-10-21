import * as lib from "@/lib";

export * as messages from "./messages";
export { Http } from "./http";
export { WebSocket } from "./websocket";

type Message = lib.notes.network.messages.Message;

export interface Network {
  send(message: Message): void;
  subscribe(listener: (message: Message) => void): () => void;
}
