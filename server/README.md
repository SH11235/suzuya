## Create Migration File

```
sea-orm-cli migrate generate "table name"
>>
Generating new migration...
Creating migration file `./migration/src/m20221105_065219_"table name".rs`
Adding migration `m20221105_065219_"table name"` to `./migration/src/lib.rs`
```

## Generate Entity File

```sh
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:8765/suzuya -o src/entity_new
```
