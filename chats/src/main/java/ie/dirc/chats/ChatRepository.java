package ie.dirc.chats;

import java.util.List;
import java.util.UUID;

import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.CrudRepository;
import org.springframework.data.repository.query.Param;

public interface ChatRepository extends CrudRepository<ChatEntity, UUID> {

    @Query("select chat from ChatEntity chat join chat.users username where username LIKE CONCAT('%', :username, '%') ")
    List<ChatEntity> findContainingUser(@Param("username") String username);
}
