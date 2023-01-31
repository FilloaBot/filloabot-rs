### Builder
FROM rust:1.64 as builder

RUN apt-get update && apt-get install -y cmake

WORKDIR /usr/src/filloabot-rs
COPY . .

RUN cargo install --path .


### Actual image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ffmpeg libopus-dev curl python && rm -rf /var/lib/apt/lists/*

RUN curl -L 'https://yt-dl.org/latest/youtube-dl' -o /usr/bin/youtube-dl
RUN chmod +x /usr/bin/youtube-dl

COPY --from=builder /usr/local/cargo/bin/filloabot-rs /usr/local/bin/filloabot-rs

CMD ["filloabot-rs"]
