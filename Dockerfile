FROM rust:latest AS builder
WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/backend* target/release/backend* src/main.rs

COPY .sqlx .sqlx
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

EXPOSE 3000
CMD ["backend"]
