FROM rust:slim-buster as builder

RUN USER=root cargo new --bin uwuki
WORKDIR /uwuki

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.lock Cargo.toml ./
COPY uwuki_macros ./uwuki_macros

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/uwuki*
RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates libssl-dev

COPY --from=builder /uwuki/target/release/uwuki /bin/uwuki

ENV RUST_LOG debug

CMD ["/bin/uwuki"]
