use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::settings::DatabaseSettings;

pub async fn connect(settings: DatabaseSettings) -> eyre::Result<PgPool> {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&settings.url)
		.await
		.map_err(|err| err.into())
}
