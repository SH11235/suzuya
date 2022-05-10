# suzuya
業務管理アプリ


## version

- DB：PostgreSQL（14）
- ORM：SeaORM（0.7）
- Backend：Actix-Web（4）
- Template：Tera（1.15.0）

## develop env

copy .env
```
cp .env.sample .env
```

PostgreSQL
```
docker-compose up -d postgres
```

install sea-orm-cli
```
cargo install sea-orm-cli
```

migration
```
sea-orm-cli migrate up
```

App start
```
cargo run
```
