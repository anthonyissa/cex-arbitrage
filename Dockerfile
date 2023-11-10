FROM rust:1.72.0-alpine

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

CMD ["./target/debug/cex-arbitrage"]