import { put } from '@/api';
import { useAuth } from '@/hooks/useAuth';
import { useChat } from '@/hooks/useChat';
import { useRouter } from 'next/router';

const Page = () => {
  const router = useRouter();
  const chatId = router.query.slug;

  const [, chat] = useChat(chatId as string);
  const { name } = chat;

  const { user } = useAuth();

  const onJoin = async () => {
    await put(
      `chats/${chatId}/join`,
      {},
      {
        headers: {
          Authorization: `Bearer ${user?.token}`,
        },
      },
    );

    router.push('/chats');
  };

  return (
    <div className="flex items-center justify-center h-screen">
      <div className="bg-base-200 px-5 py-5 rounded-xl">
        <p className="mb-5"><i>You&apos;ve been invited to join a chat</i></p>
        <div className="flex flex-col items-center justify-center">
          <h1 className="mb-2"><b>{name}</b></h1>
          <button className="btn btn-large btn-wide btn-primary" onClick={onJoin}>
            Join
          </button>
        </div>
      </div>
    </div>
  );
};

export default Page;
