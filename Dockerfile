# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder

WORKDIR /uwuki

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./uwuki_macros ./uwuki_macros
COPY ./src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=./target \
    cargo build --release

# Other image cannot access the target folder.
RUN --mount=type=cache,target=./target \
    cp ./target/release/uwuki /usr/local/bin/uwuki

FROM debian:buster-slim

COPY --from=builder /usr/local/bin/uwuki /bin/uwuki

CMD ["/bin/uwuki"]
