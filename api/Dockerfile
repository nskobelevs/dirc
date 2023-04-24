FROM maven:latest as install
ENV HOME=/usr/src/api
WORKDIR $HOME
ADD pom.xml $HOME/pom.xml
RUN mvn dependency:go-offline

FROM maven:latest as build
ENV HOME=/usr/src/api
WORKDIR $HOME
ADD . $HOME
COPY --from=install /root/.m2 /root/.m2
RUN mvn package

FROM openjdk:17-jdk
COPY --from=build /usr/src/api/target/api-0.0.1.jar /app/api.jar
ENTRYPOINT java -jar /app/api.jar