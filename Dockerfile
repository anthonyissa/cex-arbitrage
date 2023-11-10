FROM rust:1.72.0-alpine

WORKDIR /usr/src/myapp

COPY . .

RUN apt-get install pkg-config libssl-dev

RUN cargo build

CMD ["./target/debug/cex-arbitrage"]