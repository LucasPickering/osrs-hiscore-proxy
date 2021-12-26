# Actual version doesn't matter here since we use a nightly one
FROM rust:slim AS builder

WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    && \
    rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/osrs-hiscore-proxy .
COPY ./Rocket.toml ./
CMD ["/app/osrs-hiscore-proxy"]
