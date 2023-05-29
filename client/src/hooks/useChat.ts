import { useEffect, useState } from 'react';
import { type ChatInfoWithLatestMessage, type ChatInfo, type ChatMessage } from '@/types/chat';
import { get } from '@/api';
import { useAuth } from './useAuth';

export const useChat = (id: string) => {
  const { user } = useAuth();

  const [isLoading, setIsLoading] = useState(true);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [info, setInfo] = useState<ChatInfo | {}>({});

  useEffect(() => {
    const fetchMessages = async (id: string) => {
      try {
        const messages = await get<ChatMessage[]>(`chats/${id}/messages`, {
          headers: {
            Authorization: `Bearer ${user?.token}`,
          },
        });

        setMessages(messages);
      } catch (e) {
        console.log(e);
      }
    };

    const fetchInfo = async (id: string) => {
      try {
        const info = await get<ChatInfo>(`chats/${id}`, {
          headers: {
            Authorization: `Bearer ${user?.token}`,
          },
        });

        setInfo(info);
      } catch (e) {
        console.log(e);
      }
    };

    setIsLoading(true);
    fetchMessages(id)
      .then(() => fetchInfo(id))
      .then(() => setIsLoading(false))
      .catch(() => {

      });
  }, [user, id]);

  const chat = {
    ...info,
    messages,
  };

  return [isLoading, chat, setMessages] as const;
};

export const useChats = () => {
  const { user } = useAuth();
  const [isLoading, setIsLoading] = useState(true);
  const [chats, setChats] = useState<ChatInfoWithLatestMessage[]>([]);

  useEffect(() => {
    if (!user) return;

    const fetchChats = async () => {
      const chats = await get<ChatInfoWithLatestMessage[]>('chats/mychats', {
        headers: {
          Authorization: `Bearer ${user?.token}`,
        },
      });

      setChats(chats ?? []);
    };

    setIsLoading(true);
    fetchChats().then(() => setIsLoading(false));
  }, [user]);

  return [isLoading, chats, setChats] as const;
};
