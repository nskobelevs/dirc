FROM maven:latest as install
ENV HOME=/usr/src/chats
WORKDIR $HOME
ADD pom.xml $HOME/pom.xml
RUN mvn dependency:go-offline

FROM maven:latest as build
ENV HOME=/usr/src/chats
WORKDIR $HOME
ADD . $HOME
COPY --from=install /root/.m2 /root/.m2
RUN mvn package -Dmaven.test.skip

FROM openjdk:17-jdk
COPY --from=build /usr/src/chats/target/chats-0.0.1.jar /app/chats.jar
ENTRYPOINT java -jar /app/chats.jar