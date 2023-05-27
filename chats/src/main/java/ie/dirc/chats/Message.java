package ie.dirc.chats;

import java.util.UUID;

import jakarta.persistence.Embeddable;
import jakarta.persistence.GeneratedValue;

@Embeddable
public class Message {

    @GeneratedValue
    private UUID id;

    private String sender;
    private String content;

    private long timestamp;

    public Message() {
    }

    public String getSender() {
        return sender;
    }

    public String getContent() {
        return content;
    }

    public void setSender(String sender) {
        this.sender = sender;
    }

    public void setContent(String content) {
        this.content = content;
    }

    public UUID getId() {
        return id;
    }

    public long getTimestamp() {
        return timestamp;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public void setTimestamp(long timestamp) {
        this.timestamp = timestamp;
    }
}
