package ie.dirc.chats;

import java.util.UUID;

public class MessageDTO {

    private String content;

    public MessageDTO() {
    }

    public MessageDTO(String content) {
        this.content = content;
    }

    public String getContent() {
        return content;
    }

    public void setContent(String content) {
        this.content = content;
    }

    public Message toMessage(String sender) {
        Message message = new Message();
        message.setId(UUID.randomUUID());
        message.setSender(sender);
        message.setContent(content);
        message.setTimestamp(System.currentTimeMillis() / 1000);
        return message;
    }

}
