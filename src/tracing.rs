use opentelemetry::{
	runtime,
	sdk::{trace, Resource},
	KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use tonic::{metadata::MetadataMap, transport::ClientTlsConfig};
use tracing_subscriber::prelude::*;

use crate::settings::TracingSettings;

pub fn install(settings: TracingSettings) -> eyre::Result<()> {
	let mut metadata = MetadataMap::new();
	metadata.insert(
		"x-honeycomb-team",
		settings
			.api_key
			.parse()
			.expect("Could not parse setting tracing.api_key."),
	);

	let tracer = opentelemetry_otlp::new_pipeline()
		.tracing()
		.with_exporter(
			opentelemetry_otlp::new_exporter()
				.tonic()
				.with_endpoint(settings.endpoint)
				.with_tls_config(ClientTlsConfig::new())
				.with_metadata(metadata),
		)
		.with_trace_config(
			trace::config().with_resource(Resource::new(vec![KeyValue::new(
				"service.name",
				settings.service_name,
			)])),
		)
		.install_batch(runtime::Tokio)?;

	let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
	tracing_subscriber::registry().with(telemetry).try_init()?;

	Ok(())
}
