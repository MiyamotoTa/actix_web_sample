actix_web_sample
---

# Overview
actix-web sample

## Migration

### Requirement

#### diesel_cli
Migrationには `diesel_cli` を使用します。
以下のコマンドでインストールしてください。

```shell script
$ cargo install diesel_cli
```

### Set up

`.env` に接続先の設定を記載し、Diesel CLI の設定を行います。

```shell script
$ echo DATABASE_URL=mysql://root:example@localhost:13306/actix_web_sample > .env
$ diesel setup
```

### Generate migration file

以下のコマンドでMigrationファイルを作成します。

```shell script
$ diesel migration generate crate_users_table

# Creating migrations/2020-09-06-113404_create_users_table/up.sql
# Creating migrations/2020-09-06-113404_create_users_table/down.sql
```

### Apply new migration

以下のコマンドでMigrationファイルを適用します。

```shell script
$ diesel migration run

# Running migration 2020-09-06-113404_create_users_table
```

## Middleware

### Logging
アクセスログをロギングする設定を [app/mod.rs](./src/app/mod.rs) に追加。

```rust
env_logger::from_env(Env::default().default_filter_or("info")).init();

HttpServer::new(|| App::new().wrap(Logger::default()).configure(routes::routes))
    .bind("127.0.0.1:8088")?
    .run()
    .await
``` 

### NormalizePath
リクエストパスの正規化処理を [app/mod.rs](./src/app/mod.rs) に追加。
Trailing slash なしに寄せたい場合は自作する必要がありそう。

```rust
App::new()
    .wrap(Logger::default())
    .configure(routes::routes)
    .wrap(middleware::NormalizePath)
```

### Custom 404

Custom 404は [app/error/not_found.rs](./src/app/error/not_found.rs) にて実装。

```shell script
curl --location --request GET 'http://127.0.0.1:8088/not_found'
```

```json
{
    "status": 404,
    "message": "The requested `[GET] /not_found/` was not found."
}
```

## Endpoints

### [[GET] /api/articles](./src/app/v1/articles/handler/articles_handler.rs)

リクエストを受け取り、レスポンスを返す。

```shell script
curl --location --request GET 'http://127.0.0.1:8088/api/articles'
```

```json
[
    {
        "title": "title1",
        "author_name": "author_name1"
    },
    {
        "title": "title2",
        "author_name": "author_name2"
    }
]
```

### [[GET] /api/articles/{author}/{title}](./src/app/v1/articles/handler/articles_handler.rs)

PathParameterを受け取り、レスポンスを返す。

```shell script
curl --location --request GET 'http://127.0.0.1:8088/api/articles/author_name_1/article_title'
```

```json
{
    "title": "article_title",
    "author_name": "author_name_1"
}
```