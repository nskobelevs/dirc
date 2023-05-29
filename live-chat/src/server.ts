import { Application, Router } from "./deps.ts";
import { Action, Message } from "./types.ts";
import { getChats, join, leave, message } from "./actions.ts";

const app = new Application();
const router = new Router();

const broadcast = (sender: string, message: Message) => {
  const { chat } = message.payload;

  const members = chats.get(chat) ?? new Set();

  for (const username of members) {
    const connection = connections.get(username);

    if (!connection) continue;

    connection.send(
      JSON.stringify({
        ...message,
        payload: { ...message.payload, sender, timestamp: Date.now() / 1000 },
      }),
    );
  }
};

const authenticate = async (
  username: string,
  token: string,
): Promise<boolean> => {
  const authResponse = await fetch("http://auth:8080/authorize", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ username }),
  });

  if (authResponse.status !== 200) {
    console.error(`failed to authenticate user: ${username}`);
    return false;
  }

  return true;
};

const connections = new Map<string, WebSocket>();
const chats = new Map<string, Set<string>>();

router.get("/connect", async (ctx) => {
  const socket = ctx.upgrade();

  const username = ctx.request.url.searchParams.get("username");
  const token = ctx.request.url.searchParams.get("access_token");

  console.log(`new connection from ${username}`);

  if (!username) {
    socket.close(1008, "Username param is required");
    return;
  }

  if (!token) {
    socket.close(1008, "Authorization header is required");
    return;
  }

  if (!await authenticate(username, token)) {
    socket.close(1008, "Failed to authenticate");
    return;
  }

  connections.set(username, socket);

  const userChats = await getChats(token);

  for (const { id } of userChats) {
    if (!chats.has(id)) chats.set(id, new Set());
    chats.get(id)?.add(username);
  }

  socket.onmessage = (event) => {
    const data: Message = JSON.parse(event.data);
    const { action, payload } = data;

    broadcast(username, data);

    switch (action) {
      case Action.MESSAGE: {
        const { chat, content } = payload;
        message(token, content, chat);
        break;
      }
      case Action.JOIN: {
        console.log(`user ${username} joined chat ${payload.chat}`);
        const { chat } = payload;
        chats.get(chat)?.add(username);
        join(token, chat);
        break;
      }
      case Action.LEAVE: {
        const { chat } = payload;
        chats.get(chat)?.delete(username);
        if (chats.get(chat)?.size === 0) chats.delete(chat);
        leave(token, chat);
        break;
      }
    }
  };

  socket.onclose = () => {
    connections.delete(username);

    for (const [id, chat] of chats.entries()) {
      chat.delete(username);
      if (chat.size === 0) chats.delete(id);
    }

    console.log(`closed connection from ${username}`);
  };
});

app.use(router.routes());
app.use(router.allowedMethods());

export default app;
