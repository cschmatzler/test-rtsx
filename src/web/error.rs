use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebError {
	#[error("database error")]
	DatabaseError(#[from] sqlx::Error),
	#[error("internal server error")]
	InternalServerError,
}

impl IntoResponse for WebError {
	fn into_response(self) -> Response {
		match self {
			WebError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
			WebError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
		}
	}
}
