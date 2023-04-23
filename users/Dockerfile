FROM lukemathwalker/cargo-chef:latest-rust-1.68-alpine AS chef
WORKDIR /usr/src/users

FROM chef AS planner
COPY users .
COPY core-rs ../core-rs
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/users/recipe.json recipe.json
COPY core-rs ../core-rs
RUN cargo chef cook --recipe-path recipe.json
COPY users .
RUN cargo build --bin users

FROM debian:bullseye-slim AS runtime
WORKDIR /usr/src/users
COPY --from=builder /usr/src/users/target/debug/users /usr/local/bin
ENTRYPOINT ["/usr/local/bin/users"]
