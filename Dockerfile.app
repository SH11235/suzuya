FROM rust:1.60.0-slim-buster AS build-stage

ENV LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

WORKDIR /app

RUN USER=root cargo new suzuya
WORKDIR /app/suzuya

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . .
RUN rm ./target/release/deps/at_api*
RUN cargo build --release
RUN cargo install sea-orm-cli

# production
FROM debian:buster-slim AS production
RUN apt-get update
RUN apt-get install libpq-dev -y
COPY --from=build-stage /app/suzuya/target/release/suzuya .
CMD ["./suzuya"]

# dev
FROM rust:1.60.0 AS develop
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install sea-orm-cli
COPY . .
