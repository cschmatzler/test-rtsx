use std::net::SocketAddr;

use anthill::{database, router, settings::Settings, tracing};
use axum::Extension;
use listenfd::ListenFd;
use tokio::signal;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	let settings = Settings::new()?;

	color_eyre::install()?;
	if settings.tracing.enable {
		tracing::install(settings.tracing)?;
	}

	let database_pool = database::connect(settings.database).await?;

	let router = router::router()
		.layer(Extension(database_pool))
		.layer(TraceLayer::new_for_http());

	// Bind to systemfd listener if available - this is used in conjunction with
	// `just watch` for automatic reloading. If the server is not started that way,
	// bind to :3000.
	let mut listenfd = ListenFd::from_env();
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let server = match listenfd
		.take_tcp_listener(0)
		.expect("Unable to get TCP listener.")
	{
		Some(listener) => axum::Server::from_tcp(listener)
			.expect("Unable to create axum server from TCP listener."),
		None => axum::Server::bind(&addr),
	};

	server
		.serve(router.into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.expect("Unable to start server.");

	Ok(())
}

async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("Failed to install CTRL+C signal handler.");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("Failed to install UNIX termination signal handler.")
			.recv()
			.await;
	};

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	opentelemetry::global::shutdown_tracer_provider();
}

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
		assert_eq!(1, 1);
	}

	#[test]
	fn test2() {
		assert_eq!(5, 5);
	}

	#[test]
	fn test3() {
		assert_eq!(5, 5);
	}
}
