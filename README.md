# actix_web_sample

# Overview
actix-web sample

## Features

- API (actix-web)
- Create user (sqlx/MySQL)

## Endpoints

- [users](./src/app/v1/users/README.md)
- [articles](./src/app/v1/articles/README.md)

## Install

### Migration

Migrationには `diesel_cli` を使用します。
以下のコマンドでインストールしてください。

```shell script
$ cargo install diesel_cli
```

#### Set up

`.env` に接続先の設定を記載し、Diesel CLI の設定を行います。

```shell script
$ echo DATABASE_URL=mysql://root:example@localhost:13306/actix_web_sample > .env
$ diesel setup
```

#### Generate migration file

以下のコマンドでMigrationファイルを作成します。

```shell script
$ diesel migration generate crate_users_table

# Creating migrations/2020-09-06-113404_create_users_table/up.sql
# Creating migrations/2020-09-06-113404_create_users_table/down.sql
```

#### Apply new migration

以下のコマンドでMigrationファイルを適用します。

```shell script
$ diesel migration run

# Running migration 2020-09-06-113404_create_users_table
```

#### Revert migration

以下のコマンドで適用したマイグレーションをRevertします。

```shell script
$ diesel migration revert

# Rolling back migration 2020-09-06-113404_create_users_table
```