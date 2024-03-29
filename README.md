# suzuya

A merchandise management application using SeaORM, Actix-Web, Yew

## versions

- rustc：1.60.0
- DB：PostgreSQL（14）
- ORM：SeaORM（0.9）
- Backend：Actix-Web（4）
- Frontend：Yew（0.19）

## develop env

copy .env

```sh
cp .env.sample .env
```

PostgreSQL

```sh
docker compose up -d postgres
```

install sea-orm-cli

```sh
cargo install sea-orm-cli
```

migration

```sh
sea-orm-cli migrate up
```

sample data input

```sh
PGPASSWORD=postgres psql -f ./sample_data.sql -p 8765 -U postgres -d suzuya -h localhost
```

## Yew

<https://yew.rs/ja/docs/getting-started/introduction>

## App start

- server

  ```sh
    cd server
    cargo run
  ```

- client

  ```sh
    cd client
    trunk serve
  ```

## Release

- client

  ```sh
    cd client
    # dist以下にhtml, css, js, wasm生成
    trunk build --release
  ```
