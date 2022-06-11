use axum::{
    response::IntoResponse, Json,
};

use crate::Response;


pub enum AppErrorType {
    Ok,
    DbError,
    NotFound,
}

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