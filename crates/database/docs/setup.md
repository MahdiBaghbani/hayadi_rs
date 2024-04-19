You have to install SeaORM CLI for migration tasks.

```shell
cargo install sea-orm-cli
```

```shell
sea-orm-cli migrate generate users 
```

```shell
sea-orm-cli migrate -u postgres://hayadi:hayadi@127.0.0.1:5432/hayadi
```

```shell
cd entity/src
sea-orm-cli generate entity -u postgres://hayadi:hayadi@127.0.0.1:5432/hayadi
```
