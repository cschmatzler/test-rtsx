use axum::response::IntoResponse;
use maud::html;

use super::layout;

pub async fn handle_get() -> impl IntoResponse {
	layout::render(
		"Hello!",
		html! {
			h1 ."text-3xl underline text-bold text-green-700" { "Hello, World!" }
		},
	)
}
