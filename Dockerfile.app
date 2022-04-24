FROM rust:1.60.0-slim-buster

ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     libpq-dev \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     curl \
     build-essential \
     git \
     wget \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  \
  && echo "install rust tools" \
  && rustup component add rustfmt \
  && cargo install cargo-watch cargo-make sea-orm-cli \
  && cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /app

CMD ["cargo", "run"]
