actix_web_sample
---

# Overview
actix-web sample

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