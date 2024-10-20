mod error;
mod matrix;
mod sql;

use anyhow::Context;
use dioxus::prelude::*;
use dotenv::dotenv;
use envconfig::Envconfig;
use log::info;
use once_cell::sync::OnceCell;
use sql::init_pool;
use std::process::exit;
use tokio::{net::TcpListener, try_join};

use crate::app::App;

#[derive(Debug, Envconfig)]
struct  Config {
	/// the base url of the server
	base_url: String
}

static CONFIG: OnceCell<Config> = OnceCell::new();
fn config() -> &'static Config {
	CONFIG.get().unwrap()
}


pub fn main_server() {
	dotenv().ok();
	my_env_logger_style::just_log();

	if let Err(error) = main_tokio() {
		eprintln!("Error:\n{error:?}");
		exit(1);
	}
}

#[tokio::main]
async fn main_tokio() -> anyhow::Result<()> {
	CONFIG.set(Config::init_from_env()?).unwrap();
	// setup database
	init_pool().await?;

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
