import {
  type Message, type ChatMessage, Action,
} from '@/types/chat';

import React, { FormEventHandler, useEffect, useRef } from 'react';
import { useAuth } from '@/hooks/useAuth';
import { useChat } from '@/hooks/useChat';
import moment from 'moment';
import toast from 'react-hot-toast';
import { put } from '@/api';
import { useRouter } from 'next/router';

type ItemProps = {
  right: boolean;
  content: string;
  username: string;
    timestamp: string;
};

const Item = ({
  right, content, username, timestamp,
}: ItemProps) => {
  if (right) {
    return (
       <div className="w-full self-end">
        <div className="chat chat-end">
          <div className="chat-header">
              {username}
            <time className="text-xs opacity-50"> {timestamp}</time>
          </div>
          <div className="chat-bubble chat-bubble-primary">{content}</div>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full self-end">
      <div className="chat chat-start">
        <div className="chat-header">
            {username}
          <time className="text-xs opacity-50"> {timestamp}</time>
        </div>
        <div className="chat-bubble">{content}</div>
      </div>
    </div>
  );
};

type ChatProps = {
  current: string;
  latest: ChatMessage & { chatId: string } | null;
  send: (message: Message) => void;
};

const Chat = ({ current, send, latest }: ChatProps) => {
  const { user } = useAuth();
  const [isLoading, chat, setChat] = useChat(current);
  const ref = useRef<HTMLDivElement>(null);
  const [content, setContent] = React.useState('');
  const router = useRouter();

  useEffect(() => {
    if (latest) {
      setChat((prev) => {
        if (latest.chatId === current && !prev.map((m) => m.id).includes(latest.id)) {
          return [...prev, latest];
        }

        return prev;
      });
    }
  }, [latest, chat, current, setChat]);

  useEffect(() => {
    if (!isLoading) ref.current?.scrollTo(0, ref.current.scrollHeight);
  }, [current, isLoading, chat]);

  const submitMessage: FormEventHandler<HTMLFormElement> = (event) => {
    event.preventDefault();

    if (!content) return;

    send({
      action: Action.MESSAGE,
      payload: {
        content,
        chat: current,
      },
    });

    setContent('');
  };

  const onInvite = () => {
    toast('Link copied to clipboard!', { icon: '📋' });

    const link = `http://localhost:3000/join/${current}`;

    navigator.clipboard.writeText(link);
  };

  const onLeave = async () => {
    await put(`chats/${current}/leave`, {}, {
      headers: {
        Authorization: `Bearer ${user?.token}`,
      },
    });

    router.push('/chats');
  };

  return (
    <>
      <div className="rounded-tr-[25px] w-ful">
        <div className="flex gap-3 p-3 items-center">
          <div className="flex justify-between w-full">
            <span className="font-semibold text-gray-600 text-base">{chat.name}</span>
            <div>
            <button className="btn btn-sm btn-secondary mx-5" onClick={onInvite}>
              Invite +
            </button>
            <button className="btn btn-sm btn-alert" onClick={onLeave}>
              Leave Chat
            </button>
            </div>
          </div>
        </div>
        <hr className="bg-[#F0EEF5]" />
      </div>
      {isLoading && (
        <p className="px-4 text-slate-500">Loading conversation...</p>
      )}
      {!isLoading && (<div
        className="w-full h-[85vh] p-4 space-y-4 overflow-auto no-scrollbar"
        ref={ref}
      >
        {chat.messages.map(({
          id, sender, content, timestamp,
        }) => (
          <Item
            right={sender === user?.username}
            content={content}
            username={sender}
            timestamp={moment(timestamp * 1000).fromNow()}
            key={id}
          />
        ))}
      </div>)}
      <div className='w-full pt-10'>
        <form onSubmit={submitMessage} className='flex w-full mx-3 gap-2 pr-5'>
          <input
            name="message"
            className='input input-bordered input-primary flex-1'
            placeholder='Type your message here...'
            value={content}
            onChange={(event) => setContent(event.target.value)}
          />
          <button type='submit' className='btn btn-primary flex-5'>Send</button>
        </form>
      </div>
    </>
  );
};

export default Chat;
