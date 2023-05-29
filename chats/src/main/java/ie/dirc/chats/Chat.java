package ie.dirc.chats;

import java.util.List;
import java.util.UUID;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.Id;

@Entity
public class Chat {
    @Id
    @GeneratedValue
    private UUID id;
    private String name;
    private List<String> users;

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
}
