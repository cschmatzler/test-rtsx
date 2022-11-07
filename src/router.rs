use axum::{
	http::{header, HeaderMap},
	response::IntoResponse,
	routing::get,
	Router,
};

use crate::web;

const STYLES: &str = include_str!("../_assets/styles.css");

pub fn router() -> Router {
	Router::new()
		.route("/assets/styles.css", get(serve_styles))
		.route("/", get(web::dashboard::handle_get))
		.route("/health", get(web::health::handle_get))
}

async fn serve_styles() -> impl IntoResponse {
	let mut headers = HeaderMap::new();
	headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
	(headers, STYLES)
}
