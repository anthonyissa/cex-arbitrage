FROM rust:1.63.0

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

CMD ["myapp"]