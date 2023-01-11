use std::fmt;
use diesel::r2d2::PoolError;
use actix_web::{Error,ResponseError,error::BlockingError,HttpResponse,http::StatusCode};
use serde::{Serialize};

#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
//    BadReqError,
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl From<PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}
impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}
impl From<BlockingError> for AppError {
    fn from(error: BlockingError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            // AppErrorType::BadReqError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl AppError {
    // we are handling the none. the function name should match the field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}
