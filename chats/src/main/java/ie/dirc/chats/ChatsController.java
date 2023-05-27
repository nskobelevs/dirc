package ie.dirc.chats;

import java.net.URI;
import java.net.URISyntaxException;
import java.util.List;
import java.util.UUID;

import org.json.JSONObject;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

@RestController
public class ChatsController {

    private final ChatRepository chatRepository;

    @Autowired
    public ChatsController(ChatRepository chatRepository) {
        this.chatRepository = chatRepository;
    }

    @PostMapping("/{id}/send")
    public ResponseEntity<Message> sendMessage(@PathVariable("id") UUID id,
            @RequestHeader("Authorization") String token,
            @RequestBody MessageDTO messageDTO) throws URISyntaxException {
        String username = getUsernameFromAuthHeader(token);
        if (username == null) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        ChatEntity chat = chatRepository.findById(id).orElse(null);

        if (chat == null) {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);
        }

        if (!chat.containsUser(username)) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        Message msg = messageDTO.toMessage(username);

        chat.sendMessage(msg);

        chatRepository.save(chat);
        return new ResponseEntity<>(msg, HttpStatus.CREATED);
    }

    @GetMapping("/{id}")
    public ResponseEntity<ChatInfo> getChatById(@PathVariable("id") UUID id) {
        ChatEntity chat = chatRepository.findById(id).orElse(null);

        if (chat == null) {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);

        }

        return new ResponseEntity<>(chat.getInfo(), HttpStatus.OK);
    }

    @GetMapping("/{id}/messages")
    public ResponseEntity<List<Message>> getChatMessages(@PathVariable("id") UUID id,
            @RequestHeader("Authorization") String token) throws URISyntaxException {

        ChatEntity chat = chatRepository.findById(id).orElse(null);

        if (chat == null) {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);
        }

        String username = getUsernameFromAuthHeader(token);
        if (username == null || !chat.containsUser(username)) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        return new ResponseEntity<>(chat.getMessages(), HttpStatus.OK);
    }

    @GetMapping("/{id}/users")
    public ResponseEntity<List<String>> getChatUsers(@PathVariable("id") UUID id,
            @RequestHeader("Authorization") String token) throws URISyntaxException {

        ChatEntity chat = chatRepository.findById(id).orElse(null);

        if (chat == null) {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);
        }

        String username = getUsernameFromAuthHeader(token);
        if (username == null || !chat.containsUser(username)) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        return new ResponseEntity<>(chat.getUsers(), HttpStatus.OK);
    }

    @PutMapping("/create")
    public ResponseEntity<ChatInfo> createChat(@RequestHeader("Authorization") String token,
            @RequestBody ChatInfo chatInfo) throws URISyntaxException {
        System.out.println("/create");

        String username = getUsernameFromAuthHeader(token);
        if (username == null) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        ChatEntity chat = new ChatEntity(chatInfo);

        chat.addUser(username);

        chatRepository.save(chat);
        return new ResponseEntity<>(chat.getInfo(), HttpStatus.CREATED);
    }

    @GetMapping("/mychats")
    public ResponseEntity<List<ChatInfo>> getChatsByUser(@RequestHeader("Authorization") String token)
            throws URISyntaxException {
        String username = getUsernameFromAuthHeader(token);
        if (username == null) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        List<ChatEntity> chats = chatRepository.findContainingUser(username);

        List<ChatInfo> chatInfo = chats.stream().map(chat -> chat.getInfo()).toList();

        return new ResponseEntity<>(chatInfo, HttpStatus.OK);
    }

    @PutMapping("/{id}/join")
    public ResponseEntity<ChatInfo> joinChat(@PathVariable("id") UUID id,
            @RequestHeader("Authorization") String token) throws URISyntaxException {

        // TODO: Change to slug and use url service
        ChatEntity chat = chatRepository.findById(id).orElse(null);
        if (chat == null) {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);
        }

        String username = getUsernameFromAuthHeader(token);
        if (username == null) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        chat.addUser(username);
        chatRepository.save(chat);

        return new ResponseEntity<>(chat.getInfo(), HttpStatus.OK);
    }

    @PutMapping("/{id}/leave")
    public ResponseEntity<ChatInfo> leaveChat(@PathVariable("id") UUID id,
            @RequestHeader("Authorization") String token) throws URISyntaxException {

        ChatEntity chat = chatRepository.findById(id).orElse(null);
        if (chat == null) {
            return new ResponseEntity<>(null, HttpStatus.NOT_FOUND);
        }

        String username = getUsernameFromAuthHeader(token);
        if (username == null) {
            return new ResponseEntity<>(null, HttpStatus.UNAUTHORIZED);
        }

        chat.removeUser(username);
        chatRepository.save(chat);

        return new ResponseEntity<>(chat.getInfo(), HttpStatus.OK);
    }

    private String getUsernameFromAuthHeader(String token) throws URISyntaxException {

        ResponseEntity<String> authResponse = authenticate(token);

        if (authResponse.getStatusCode() != HttpStatus.OK) {
            System.out.println("Error: " + authResponse.getBody());
            return null;
        }

        JSONObject authResponseBody = new JSONObject(authResponse.getBody());
        String username = authResponseBody.getString("username");

        return username;
    }

    private ResponseEntity<String> authenticate(String token) throws URISyntaxException {
        URI uri = new URI("http://auth:8080/authenticate");

        HttpHeaders headers = new HttpHeaders();
        headers.setContentType(MediaType.APPLICATION_JSON);
        headers.set("Authorization", token);

        RestTemplate restTemplate = new RestTemplate();
        HttpEntity<String> request = new HttpEntity<>(headers);

        return restTemplate.exchange(uri, HttpMethod.GET, request, String.class);
    }
}
