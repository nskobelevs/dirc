import Chat from '@/components/Chat';
import Sidebar from '@/components/Sidebar';
import { useChats } from '@/hooks/useChat';
import {
  Action, ChatMessage, SocketResponse,
} from '@/types/chat';
import { useEffect, useState } from 'react';
import { useWebsocket } from '@/hooks/useWebsocket';

const Chats = () => {
  const [, chats, setChats] = useChats();
  const [selected, setSelected] = useState<string | null>(chats[0]?.id ?? null);
  const [latest, setLatest] = useState<ChatMessage & { chatId: string } | null>(null);

  useEffect(() => {
    if (chats.length === 0) {
      setSelected(null);
    }
  }, [chats]);

  const receiveMessage = (message: SocketResponse) => {
    if (message.action === Action.MESSAGE) {
      const {
        chat: id, sender, content, timestamp,
      } = message.payload;

      setChats((prev) => {
        const chat = prev.find((c) => c.id === id);

        if (chat) {
          const chatMessage = {
            sender,
            content,
            timestamp,
            id: timestamp.toString(),
          };

          const newChat = {
            ...chat,
            latestMessage: chatMessage,
          };

          setLatest({ ...chatMessage, chatId: id });

          return [newChat, ...prev.filter((c) => c.id !== id)];
        }

        return prev;
      });
    }
  };

  const { send } = useWebsocket(receiveMessage);

  return (
    <main className='flex w-full mx-auto bg-[#FAF9FE] backdrop-opacity-30 opacity-95'>
    <aside className='bg-[#F0EEF5] w-[325px] min-w-[325px] h-[100vh] rounded-l-[25px] p-4 relative'>
      <Sidebar chats={chats} selected={selected} onSelect={setSelected} send={send} />
    </aside>
    {selected && (<section className='flex w-full flex-col h-[100vh]'>
      <Chat current={selected} send={send} latest={latest} />
    </section>)}
  </main>
  );
};

export default Chats;
