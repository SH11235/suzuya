FROM rust:1.60.0-slim-buster AS build-stage

RUN USER=root cargo new --bin suzuya
RUN apt-get update \
    && apt-get install -y -q \
    ca-certificates \
    locales \
    gnupg \
    apt-transport-https\
    libssl-dev \
    pkg-config \
    build-essential \
    git \
    wget \
    && echo "ja_JP UTF-8" > /etc/locale.gen  \
    && locale-gen

WORKDIR /suzuya
COPY ./server .
RUN cargo build --release

# production
FROM debian:bullseye-slim AS production

RUN apt-get update  \
    && apt-get install -y -q \
    curl

# Copy build file
COPY --from=build-stage /suzuya/target/release/suzuya .
COPY --from=build-stage /suzuya/.env .

WORKDIR /
CMD ["./suzuya"]
