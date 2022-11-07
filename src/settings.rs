use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
	pub tracing: TracingSettings,
	pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct TracingSettings {
	pub enable: bool,
	pub endpoint: String,
	pub service_name: String,
	pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
	pub url: String,
}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());

		let config = Config::builder()
			.add_source(File::with_name("settings/base"))
			.add_source(File::with_name(&format!("settings/{}", run_env)).required(false))
			.add_source(Environment::with_prefix("ANTHILL"))
			.build()?;

		config.try_deserialize()
	}
}
