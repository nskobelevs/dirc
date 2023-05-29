import { put } from '@/api';
import { useAuth } from '@/hooks/useAuth';
import {
  ChatInfoWithLatestMessage,
  ChatInfo,
  Message,
  Action,
} from '@/types/chat';
import moment from 'moment';
import { useRouter } from 'next/router';
import React, { useState } from 'react';

type ItemProps = {
  onSelect: (key: string) => void;
  chat: ChatInfoWithLatestMessage;
  selected: boolean;
};

const Item = ({ chat, onSelect, selected }: ItemProps) => {
  const { id, name, latestMessage } = chat;
  const { content, timestamp } = latestMessage ?? {
    sender: null,
    content: 'No messages in this chat yet!',
    timestamp: null,
  };
  const date = timestamp ? moment(timestamp * 1000) : null;

  return (
    <div
      onClick={() => onSelect(id)}
      className={`${
        selected
          ? 'bg-[#FDF9F0] border border-[#DEAB6C]'
          : 'bg-[#FAF9FE] border border-[#FAF9FE]'
      } p-2 rounded-[10px] shadow-sm cursor-pointer`}
    >
      <div className="flex justify-between items-center gap-1">
        <div className="flex gap-3 items-center w-full">
          <div className="w-full max-w-[150px]">
            <h3 className="font-semibold text-sm text-gray-700">{name}</h3>
            <p className="font-light text-xs text-gray-600 truncate">
              {content}
            </p>
          </div>
        </div>
        {timestamp && (
          <div className="text-gray-400 min-w-[85px]">
            <span className="text-xs">{date?.fromNow() ?? ''}</span>
          </div>
        )}
      </div>
    </div>
  );
};

type SidebarProps = {
  chats: ChatInfoWithLatestMessage[];
  selected: string | null;
  send: (message: Message) => void;
  onSelect: (key: string) => void;
};

const Sidebar = ({
  chats, selected, onSelect, send,
}: SidebarProps) => {
  const router = useRouter();

  const { logout, user } = useAuth();

  const [name, setName] = useState('');

  const createNewChat = async () => {
    if (!name) return;

    const { id } = await put<ChatInfo, { name: string }>(
      'chats/create',
      { name },
      {
        headers: {
          Authorization: `Bearer ${user?.token}`,
        },
      },
    );

    send({
      action: Action.JOIN,
      payload: {
        chat: id,
      },
    });

    setName('');
    window.location.reload();
  };

  return (
    <div className="flex flex-col gap-3">
      <input
        className="input input-bordered w-full"
        placeholder="Chat Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />

      <button
        className="btn btn-accent btn-outline btn-wide w-full"
        onClick={createNewChat}
      >
        New Chat
      </button>
      <div className="overflow-auto flex flex-col gap-3 h-[80vh] no-scrollbar">
        {chats.map((chat) => (
          <Item
            key={chat.id}
            chat={chat}
            selected={selected === chat.id}
            onSelect={onSelect}
          />
        ))}
      </div>
      <button
        onClick={logout}
        className="btn btn-wide btn-outline btn-accent w-[90%] absolute bottom-4"
      >
        LOG OUT
      </button>
    </div>
  );
};

export default Sidebar;
