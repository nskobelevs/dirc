import { ChatInfo } from "./types.ts";

export const message = (token: string, content: string, id: string) => {
  fetch(`http://chats:8080/${id}/send`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ content }),
  });
};

export const join = (token: string, id: string) => {
  fetch(`http://chats:8080/${id}/join`, {
    method: "PUT",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });
};

export const leave = (token: string, id: string) => {
  fetch(`http://chats:8080/${id}/leave`, {
    method: "PUT",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });
};

export const getChats = async (token: string): Promise<ChatInfo[]> => {
  const response = await fetch("http://chats:8080/mychats", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  return response.json();
};
