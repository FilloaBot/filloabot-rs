### Builder
FROM rust:1.64 as builder

RUN apt-get update && apt-get install -y cmake

WORKDIR /usr/src/filloabot-rs
COPY . .

RUN cargo install --path .


### Actual image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ffmpeg youtube-dl libopus-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/filloabot-rs /usr/local/bin/filloabot-rs

CMD ["filloabot-rs"]