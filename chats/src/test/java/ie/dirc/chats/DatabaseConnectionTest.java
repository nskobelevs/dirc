package ie.dirc.chats;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.SQLException;

import org.junit.jupiter.api.Test;

public class DatabaseConnectionTest {
    @Test
    void connectionTest() {
        String url = "jdbc:mysql://localhost:3306/dIRC";
        String username = "mudsigmoids";
        String password = "mudsigmoids";

        try (Connection connection = DriverManager.getConnection(url, username, password)) {
            System.out.println("Database connected");
        } catch (SQLException e) {
            throw new IllegalStateException("Cannot connect to database", e);
        }
    }
}
