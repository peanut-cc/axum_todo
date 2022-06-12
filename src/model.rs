use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Clone)]
pub struct AppState {
    /// pg pool
    pub pool: deadpool_postgres::Pool,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoListID {
    pub id: i32,
}