#![allow(non_snake_case)]

mod app;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::{tracing, tracing::Level};

fn main() {
	#[cfg(feature = "web")]
	main_client();

	#[cfg(feature = "server")]
	main_server();
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main_server() {
	dioxus_logger::init(Level::INFO).expect("failed to init logger");

	let routes = axum::Router::new()
		.serve_dioxus_application(ServeConfig::builder().build(), || VirtualDom::new(App))
		.await;
	let listener = tokio::net::TcpListener::bind("localhost:8080")
		.await
		.unwrap();
	axum::serve(listener, routes.into_make_service())
		.await
		.unwrap();
}

#[cfg(feature = "web")]
fn main_client() {
	// Init logger
	dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

	// Hydrate the application on the client
	dioxus::web::launch::launch_cfg(App, dioxus::web::Config::new().hydrate(true));
}
