FROM lukemathwalker/cargo-chef:latest-rust-1.68-alpine AS chef
WORKDIR /usr/src/auth

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/auth/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN cargo build --bin auth

FROM debian:bullseye-slim AS runtime
WORKDIR /usr/src/auth
COPY --from=builder /usr/src/auth/target/debug/auth /usr/local/bin
ENTRYPOINT ["/usr/local/bin/auth"]
