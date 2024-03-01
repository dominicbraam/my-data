use std::fmt;
use diesel::r2d2::PoolError;
use diesel::result::{
    // DatabaseErrorKind,
    Error as DieselError
};
use actix_web::{
    Error,
    ResponseError,
    error::BlockingError,
    HttpResponse,
    http::StatusCode
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    ReqwestError,
//    BadReqError,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub cause: Option<String>,          // more detailed, for logging
    pub user_message: Option<String>,   // user-friendly message
    pub log_message: Option<String>,    // detailed log message
    pub error_type: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::DbError,
        }
    }
}
impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::DbError,
        }
    }
}
impl From<BlockingError> for AppError {
    fn from(error: BlockingError) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::DbError,
        }
    }
}
impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::ReqwestError,
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        match error {
            DieselError::NotFound => AppError {
                cause: Some("Resource not found in the database.".to_string()),
                user_message: Some("The requested item was not found.".to_string()),
                log_message: Some(format!("Database not found error: {:?}", error)),
                error_type: AppErrorType::NotFoundError,
            },
            DieselError::DatabaseError(_, info) => {
            // DieselError::DatabaseError(kind, info) => {
                // let kind = match kind {
                //     DatabaseErrorKind::UniqueViolation => "Unique constraint violation".to_string(),
                //     DatabaseErrorKind::ForeignKeyViolation => "Foreign key violation".to_string(),
                //     _ => "Database error".to_string(),
                // };
                AppError {
                    cause: Some(info.message().to_string()),
                    user_message: Some("Database error".to_string()),
                // log_message: Some(format!("Database not found error: {:?}", kind)),
                    log_message: Some(info.message().to_string()),
                    error_type: AppErrorType::DbError,
                }
            }
            _ => AppError {
                cause: Some(error.to_string()),
                user_message: None,
                log_message: None,
                error_type: AppErrorType::DbError,
            },
        }
    }
}

impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::ReqwestError => StatusCode::INTERNAL_SERVER_ERROR,
            // AppErrorType::BadReqError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("Error: {}", self.log_message());

        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.user_message(),
        })
    }
}

impl AppError {

    fn user_message(&self) -> String {
        self.user_message.clone().unwrap_or_else(|| match self.error_type {
            AppErrorType::NotFoundError => "The requested item was not found.".to_string(),
            _ => "An unexpected error has occurred.".to_string()
        })
    }

    fn log_message(&self) -> String {
        if let Some(ref message) = self.log_message {
            message.clone()
        } else if let Some(ref cause) = self.cause {
            cause.clone()
        } else {
            "An unexpected error occured.".to_string()
        }
    }
}
