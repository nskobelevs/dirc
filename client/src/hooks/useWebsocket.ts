import { Message, SocketResponse } from '@/types/chat';
import { useEffect, useRef } from 'react';
import { useAuth } from './useAuth';

export const useWebsocket = (onmessage: (response: SocketResponse) => void) => {
  const { user } = useAuth();

  const socket = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (!user?.token) return;

    const uri = `ws://localhost:8083/connect?access_token=${user?.token}&username=${user?.username}`;

    if (socket.current) return;

    socket.current = new WebSocket(uri);

    // return () => {
    //   socket.current?.close();
    // };
  }, [user]);

  useEffect(() => {
    if (!socket.current) return;

    socket.current.onmessage = (event) => {
      onmessage(JSON.parse(event.data));
    };
  });

  const send = (message: Message) => {
    if (!socket.current) return;

    socket.current.send(JSON.stringify(message));
  };

  return { send };
};
