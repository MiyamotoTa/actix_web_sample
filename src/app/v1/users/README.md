# users

## Features
- Create user

## Endpoints

### [[POST] /api/v1/users](./handler/users_handler.rs)

ユーザを新規作成します。

#### Request
```shell script
curl --location --request POST 'http://127.0.0.1:8088/api/v1/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "test@example.com",
    "name": "create_test_user"
}'
```

#### Response

```json
{
    "id": 1,
    "name": "create_test_user",
    "email": "test@example.com",
    "created_at": "2020-09-08T13:37:39",
    "updated_at": "2020-09-08T13:37:39"
}
```

### [[GET] /api/v1/users/{id}](./handler/users_handler.rs)

パス指定のユーザ情報を取得します。

#### Request
```shell script
curl --location --request GET 'http://127.0.0.1:8088/api/v1/users/1'
```

#### Response

```json
{
    "id": 1,
    "name": "create_test_user",
    "email": "test@example.com",
    "created_at": "2020-09-08T13:37:39",
    "updated_at": "2020-09-08T13:37:39"
}
```