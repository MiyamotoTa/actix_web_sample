actix_web_sample
---

# Overview
actix-web sample

## Endpoints

### [GET] /api/articles

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