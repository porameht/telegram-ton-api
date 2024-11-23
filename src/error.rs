use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    #[error("Resource not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] mongodb::bson::ser::Error),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] mongodb::bson::de::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::MongoDB(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::BadRequest(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
            ApiError::InternalServerError(ref message) => (StatusCode::INTERNAL_SERVER_ERROR, message.clone()),
            ApiError::Serialization(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Deserialization(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
} 