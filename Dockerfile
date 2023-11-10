FROM rust:1.63.0

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build

CMD ["./target/debug/cex-arbitrage"]