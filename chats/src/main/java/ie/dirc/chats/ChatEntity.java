package ie.dirc.chats;

import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

import jakarta.persistence.ElementCollection;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.Id;

@Entity
public class ChatEntity {
    @Id
    @GeneratedValue
    private UUID id;

    private String name;

    @ElementCollection
    private List<String> users;

    @ElementCollection
    private List<Message> messages;

    public ChatEntity() {
    }

    public ChatEntity(String name, UUID id) {
        this.name = name;
        this.users = new ArrayList<>();
        this.messages = new ArrayList<>();
        this.id = id;
    }

    public ChatEntity(ChatInfo info) {
        this(info.name, info.id);
    }

    public UUID setId(UUID id) {
        return this.id = id;
    }

    public UUID getId() {
        return id;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }

    public void addUser(String user) {
        users.add(user);
    }

    public void removeUser(String user) {
        users.remove(user);
    }

    public List<String> getUsers() {
        return users;
    }

    public List<Message> getMessages() {
        return messages;
    }

    public boolean containsUser(String username) {
        return users.contains(username);
    }

    public ChatInfo getInfo() {
        return new ChatInfo(name, id);
    }

    public void sendMessage(Message message) {
        messages.add(message);
    }
}
