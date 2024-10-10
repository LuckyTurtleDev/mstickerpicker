mod matrix;

use crate::app::App;
use anyhow::{Context, Ok};
use dioxus::prelude::*;
use dotenv::dotenv;
use log::info;
use std::process::exit;
use tokio::try_join;

pub fn main_server() {
	dotenv().ok();
	
	if let Err(error) = main_tokio() {
		eprintln!("Error:\n{error:?}");
		exit(1);
	}
}

#[tokio::main]
async fn main_tokio() -> anyhow::Result<()> {
	use tokio::net::TcpListener;
	my_env_logger_style::just_log();
	let routes = axum::Router::new()
		.serve_dioxus_application(ServeConfig::builder().build(), || VirtualDom::new(App))
		.await;
	let listener = TcpListener::bind("localhost:8080")
		.await
		.context("failed to bind port")?;
	info!("starting web server at http://localhost:8080/");

	try_join!(
		async {
			axum::serve(listener, routes.into_make_service())
				.await
				.context("failed to axum start webserver")
		},
		async {
			matrix::start_matrix()
				.await
				.context("failed to start matrix client")
		}
	)?;
	Ok(())
}
