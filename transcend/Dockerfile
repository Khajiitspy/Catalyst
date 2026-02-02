# 1. Стадія збірки
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release

# 2. Мінімальний runtime
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/my_app .

CMD ["./my_app"]
 
