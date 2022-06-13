

pub mod configuration;
pub mod handler;
pub mod error;
pub mod response;
pub mod model;
pub mod form;
pub mod db;



use axum::{Router, routing::get, extract::Extension};
use configuration::{Settings, DatabaseSettings};
use deadpool_postgres::Runtime;
use model::AppState;
use handler::{
    todo_list,
    usage
};

use crate::handler::todo_item;
pub use crate::response::Response;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> deadpool_postgres::Pool {
    let pool = configuration.with_db()
        .create_pool(Some(Runtime::Tokio1),tokio_postgres::NoTls)
        .expect("init db pool failed");
    pool
}

pub async fn run(configuration: Settings) {
    let pool = get_connection_pool(&configuration.database);
    let addr =  format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    ).parse().unwrap();
    tracing::info!("server listen: {}", &addr);
    let app = Router::new()
        .route("/", get( usage::usage))
        .route("/todo",
            get(todo_list::all)
            .post(todo_list::create),
        )
        .route("/todo/:list_id",
            get(todo_list::find)
            .put(todo_list::update)
            .delete(todo_list::delete),
        )
        .route(
            "/todo/:list_id/items",
            get(todo_item::all)
                .post(todo_item::create),
        )
        .route(
            "/todo/:list_id/items/:item_id",
            get(todo_item::find)
                .put(todo_item::check)
                .delete(todo_item::delete),
        )
        .layer(Extension(AppState { pool }));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}