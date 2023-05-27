package ie.dirc.chats;

import java.util.UUID;

public class ChatInfo {

    public String name;
    public UUID id;

    public ChatInfo(String name, UUID id) {
        this.name = name;
        this.id = id;
    }
}
