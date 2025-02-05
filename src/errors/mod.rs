use actix_web::error::JsonPayloadError;
use actix_web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use serde_derive::Serialize;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
    JsonPayloadError(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "Record already exists"),
            AppError::RecordNotFound => write!(f, "Record not found"),
            AppError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            AppError::OperationCanceled => write!(f, "Operation canceled"),
            AppError::JsonPayloadError(ref err) => write!(f, "Invalid JSON payload: {}", err),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        match error {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(error),
        }
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(error: JsonPayloadError) -> Self {
        match error {
            _ => AppError::JsonPayloadError(error.to_string()),
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }
}
