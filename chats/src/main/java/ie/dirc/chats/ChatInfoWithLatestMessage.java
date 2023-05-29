package ie.dirc.chats;

public class ChatInfoWithLatestMessage extends ChatInfo {
    public Message latestMessage;

    public ChatInfoWithLatestMessage(ChatEntity chat) {
        super(chat.getName(), chat.getId());
        if (chat.getMessages().size() == 0) {
            this.latestMessage = null;
        } else {
            this.latestMessage = chat.getMessages().get(chat.getMessages().size() - 1);
        }
    }
}
