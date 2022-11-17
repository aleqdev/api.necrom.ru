FROM rust:1.65-slim as builder

RUN USER=root cargo new --bin cached_project
WORKDIR /cached_project

RUN apt update -y && apt upgrade -y
RUN apt install -y libssl-dev pkg-config

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN mkdir .cargo
RUN cargo vendor > .cargo/config

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./templates ./templates

RUN rm -f ./target/release/deps/api_necrom_ru*
RUN cargo build --release

FROM debian:buster-slim

RUN apt update -y && apt upgrade -y
RUN apt install -y openssl ca-certificates

COPY --from=builder /cached_project/target/release/api_necrom_ru .
COPY ./user ./user
COPY ./restore ./restore

CMD ["/api_necrom_ru"]
