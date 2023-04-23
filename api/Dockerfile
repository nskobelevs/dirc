FROM openjdk:17-jdk
COPY . .
RUN ./mvnw package
CMD /usr/bin/java -jar target/api-0.0.1.jar
