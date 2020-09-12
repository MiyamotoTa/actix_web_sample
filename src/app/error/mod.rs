use std::fmt;

use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::export::Formatter;
use serde::Serialize;

pub(crate) mod not_found;

#[derive(Debug)]
pub enum AppErrorType {
    NotFoundError,
    UnProcessableEntityError,
    DbError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                cause: _,
                error_type: _,
            } => message.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::UnProcessableEntityError,
            } => "Validation failed".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }

    pub fn not_found_error(error: impl ToString) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::NotFoundError,
        }
    }

    pub fn un_processable_entity_error(error: impl ToString) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::UnProcessableEntityError,
        }
    }

    pub fn db_error(error: impl ToString) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::UnProcessableEntityError => StatusCode::UNPROCESSABLE_ENTITY,
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            code: self.status_code().as_u16(),
            message: self.message(),
            cause: self.cause.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_error_should_be_return_default_message() {
        let error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::NotFoundError,
        };

        assert_eq!(
            error.message(),
            "The requested item was not found".to_string()
        )
    }

    #[test]
    fn un_processable_entity_error_should_be_return_default_message() {
        let error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::UnProcessableEntityError,
        };

        assert_eq!(error.message(), "Validation failed".to_string())
    }

    #[test]
    fn db_error_should_be_return_default_message() {
        let error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(
            error.message(),
            "An unexpected error has occurred".to_string()
        )
    }

    #[test]
    fn custom_message_should_be_shown() {
        let custom_message = "Unable to create item".to_string();
        let db_error = AppError {
            message: Some(custom_message.clone()),
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            custom_message,
            "Custom message should be shown"
        )
    }
}
