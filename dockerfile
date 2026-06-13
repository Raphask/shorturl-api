FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo clean
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/url-shorter .
CMD ["./url-shorter"]