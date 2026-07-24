FROM rust:1.96.0-slim AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release

# --- #

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/vocab-api /app/vocab-api

EXPOSE 7222

ENTRYPOINT ["/app/vocab-api"]
