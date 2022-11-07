use axum::{http::StatusCode, response::IntoResponse, Extension};
use sqlx::PgPool;

use super::error::WebError;

pub async fn handle_get(
	Extension(database_pool): Extension<PgPool>,
) -> Result<impl IntoResponse, WebError> {
	sqlx::query("SELECT 1").fetch_one(&database_pool).await?;

	Ok(StatusCode::OK)
}
