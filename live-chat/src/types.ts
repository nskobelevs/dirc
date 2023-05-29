export enum Action {
  MESSAGE = "MESSAGE",
  JOIN = "JOIN",
  LEAVE = "LEAVE",
}

export type Message =
  | { action: Action.MESSAGE; payload: { content: string; chat: string } }
  | { action: Action.JOIN; payload: { chat: string } }
  | { action: Action.LEAVE; payload: { chat: string } };

export type ChatInfo = {
  id: string;
  name: string;
};
