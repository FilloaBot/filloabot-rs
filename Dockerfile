### Builder
FROM rust:1.64 as builder

WORKDIR /usr/src/filloabot-rs
COPY . .

RUN apt-get update && apt-get install -y cmake

RUN cargo install --path .


### Actual image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/filloabot-rs /usr/local/bin/filloabot-rs

CMD ["filloabot-rs"]