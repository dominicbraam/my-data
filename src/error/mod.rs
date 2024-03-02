use std::fmt;
use diesel::r2d2::PoolError;
use diesel::result::Error as DieselError;
use actix_web::{
    Error as ActixError,
    error::ResponseError,
    error::BlockingError,
    HttpResponse,
    http::StatusCode
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub cause: Option<String>,          // more detailed, for logging
    pub user_message: Option<String>,   // user-friendly message
    pub log_message: Option<String>,    // detailed log message
    pub error_type: AppErrorType,
}

#[derive(Debug, Serialize)]
pub enum AppErrorType {
    DatabaseError,
    NotFoundError,
    ReqwestError,
    BadReqError,
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

impl AppError {

    fn get_user_message(&self) -> String {
        self.user_message.clone().unwrap_or_else(|| match self.error_type {
            AppErrorType::NotFoundError => "The requested item was not found.".to_string(),
            _ => "An unexpected error has occurred.".to_string()
        })
    }

    fn get_log_message(&self) -> String {
        if let Some(ref message) = self.log_message {
            message.clone()
        } else if let Some(ref cause) = self.cause {
            cause.clone()
        } else {
            "An unexpected error occured.".to_string()
        }
    }

    fn get_error_type(&self) -> &AppErrorType {
        &self.error_type
    }
}

impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::ReqwestError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::BadReqError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        tracing::error!("{}: {}",self.get_error_type(), self.get_log_message());

        HttpResponse::build(self.status_code()).json(
            AppErrorResponse {
                error: self.get_user_message(),
            }
        )
    }
}

impl From<PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::DatabaseError,
        }
    }
}

impl From<ActixError> for AppError {
    fn from(error: ActixError) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::DatabaseError,
        }
    }
}
impl From<BlockingError> for AppError {
    fn from(error: BlockingError) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            user_message: None,
            log_message: None,
            error_type: AppErrorType::DatabaseError,
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
                log_message: Some("Resource not found in the database.".to_string()),
                error_type: AppErrorType::NotFoundError,
            },
            DieselError::DatabaseError(_, info) => {
                AppError {
                    cause: Some(info.message().to_string()),
                    user_message: Some("Database error".to_string()),
                    log_message: Some(info.message().to_string()),
                    error_type: AppErrorType::DatabaseError,
                }
            }
            _ => AppError {
                cause: Some(error.to_string()),
                user_message: None,
                log_message: None,
                error_type: AppErrorType::DatabaseError,
            },
        }
    }
}
