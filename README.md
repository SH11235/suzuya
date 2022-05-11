# suzuya

A merchandise management application using SeaORM, Actix-Web, Tera

## versions

- rustc：1.60.0
- DB：PostgreSQL（14）
- ORM：SeaORM（0.7）
- Backend：Actix-Web（4）
- Template：Tera（1.15.0）

## develop env

copy .env

```sh
cp .env.sample .env
```

PostgreSQL

```sh
docker-compose up -d postgres
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

App start

```sh
cargo run
```
