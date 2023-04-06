FROM rust:1.67

WORKDIR /usr/src/auth
COPY . .

RUN cargo install --path .

CMD ["auth"]
