FROM rust:1.72.0-alpine as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

RUN apt update && apt install -y pkg-config
RUN apt update && apt install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

CMD ["myapp"]