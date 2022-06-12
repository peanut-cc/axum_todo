use deadpool_postgres::Client;

use crate::{error::AppError, model::AppState,Result};

pub mod todo_list;
pub mod usage;


async fn get_client(state: &AppState, handler_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("{}: {:?}", handler_name, err);
        AppError::db_error(err)
    })
}

fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::debug!("{}: {:?}", handler_name, err);
        err
    })
}