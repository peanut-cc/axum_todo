# axum todo

Simple TODO list API made in rust

## API

 `GET` `/`

```not rust
{
    "code": 0,
    "msg": "OK",
        "data": [
        "### USAGE ###",
        "- GET /todo -- get all todo list",
        "- POST /todo -- create a todo list",
        "- GET /todo/:list_id -- get detail for a todo list",
        "- DELETE /todo/:list_id -- delete a todo list, include it's items",
        "- PUT /todo/:list_id -- edit a todo list",
        "- GET /todo/:list_id/items -- get items from todo list",
        "- GET /todo/:list_id/items/:item_id -- get detail for a todo item",
        "- PUT /todo/:list_id/items/:item_id -- edit a todo item(set the item to checked)",
        "- DELETE /todo/:list_id/items/:item_id -- delete a todo item"
    ]
}

```

`GET` `/todo`

```not rust
{
    "code": 0,
    "msg": "OK",
    "data": [
        {
            "id": 2,
            "title": "todo two"
        },
        {
            "id": 1,
            "title": "todo one"
        }
    ]
}
```

`POST` `/todo`

```not rust
reqeust:
{
    "title": "todo three"
}

response:
{
    "code": 0,
    "msg": "OK",
    "data": {
        "id": 3
    }
}
```

`GET` `/todo/:id`

```not rust

{
    "code": 0,
    "msg": "OK",
    "data": {
        "id": 1,
        "title": "todo one"
    }
}
```

`put` `/todo/:id`

```not rust
request:
{
    "id": 1,
    "title": "new one"
}

response:
{
    "code": 0,
    "msg": "OK",
    "data": true
}
```

`DELETE` `todo/1`

```not rust
{
    "code": 0,
    "msg": "OK",
    "data": true
}
```
