articles
----

## Features
- Simple get request
- Handle path parameters


## Endpoints

### [[GET] /api/articles](./handler/articles_handler.rs)

リクエストを受け取り、レスポンスを返す。

#### Request

```shell script
curl --location --request GET 'http://127.0.0.1:8088/api/articles'
```

#### Response
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

### [[GET] /api/articles/{author}/{title}](./handler/articles_handler.rs)

#### Request

```shell script
curl --location --request GET 'http://127.0.0.1:8088/api/articles/author_name_1/article_title'
```

#### Response

```json
{
    "title": "article_title",
    "author_name": "author_name_1"
}
```