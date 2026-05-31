FROM rust:slim-bookworm

MAINTAINER Henry Nguyen

WORKDIR /app
COPY . .

# Install Trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

# Update
RUN apt-get update && apt-get install -y \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64 && \
    chmod +x tailwindcss-linux-arm64 && \
    mv tailwindcss-linux-arm64 tailwindcss

EXPOSE 3000

RUN trunk build --release