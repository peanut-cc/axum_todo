use std::fmt::Display;

use axum::{
    response::IntoResponse, Json,
};

use crate::Response;

#[derive(Debug)]
pub enum AppErrorType {
    Ok,
    DbError,
    NotFound,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    fn code(&self) -> i32 {
        match self.error_type {
            AppErrorType::Ok => 0,
            AppErrorType::DbError => 1,
            AppErrorType::NotFound => 2,
        }
    }

    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }

    fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }

    pub fn db_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbError)
    }

    pub fn not_found() -> Self {
        Self::from_str("not found", AppErrorType::NotFound)
    }
}

impl IntoResponse for AppError {

    fn into_response(self) -> axum::response::Response {
        let code = (&self).code();
        let msg = match self.message {
            
            Some(msg) => msg,
            None => "error".to_string(),
        };
        let res:Response<()> = Response::err(code, msg);
        Json(res).into_response()
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::db_error(err)
    }
}
impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::db_error(err)
    }
}

impl std::error::Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}