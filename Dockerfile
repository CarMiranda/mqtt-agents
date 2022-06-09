FROM rust:1.61 AS builder

RUN USER=root cargo new --bin embagent
WORKDIR /embagent

RUN apt-get update && apt-get install -y cmake libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release
RUN rm src/*.rs

COPY src src

RUN rm target/release/deps/embagent*
RUN cargo build --release


FROM rust:1.61-slim-buster

COPY --from=builder /embagent/target/release/embagent .

CMD ["./embagent"]
