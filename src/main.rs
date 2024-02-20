use anyhow::Context;
use axum::{
	routing::{get, post},
	Router
};
use dotenv::dotenv;
use log::info;
use serde::{de, Deserialize};
use tokio::try_join;

use crate::matrix::{start_matrix, MatrixConfig};

mod components;
mod matrix;
mod routes;
mod style;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_env(var: &str) -> String {
	#![allow(clippy::expect_fun_call)]
	std::env::var(var).expect(&format!("Enviroment variable {var:?} must be set"))
}

#[tokio::main]
async fn main() {
	dotenv().ok();
	let matrix_config = MatrixConfig {
		homeserver_url: load_env("MARIX_HOMESERVER_URL"),
		username: load_env("MARIX_USERNAME"),
		password: load_env("MARIX_PASSWORD")
	};
	my_env_logger_style::just_log();
	let app = Router::new()
		.route("/", get(routes::index))
		.route("/css", get(routes::css))
		.route("/register", get(routes::register))
		.route("/register", post(routes::register_post))
		.route("/login", get(routes::login));
	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
	info!("statring web server at http://localhost:8080/");
	let res = try_join!(
		async {
			axum::serve(listener, app)
				.await
				.context("failed to start axum webserver")
		},
		async {
			start_matrix(matrix_config)
				.await
				.context("failed to start matrix client")
		}
	);
	if let Err(err) = res {
		panic!("{err:?}")
	}
}

pub fn take_first<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
	D: de::Deserializer<'de>,
	T: Deserialize<'de>
{
	let vec: Vec<T> = Vec::deserialize(deserializer)?;
	Ok(vec.into_iter().next())
}

trait ToQuery {
	fn to_query(self) -> String;
}

impl<T: ToQuery> ToQuery for Option<T> {
	fn to_query(self) -> String {
		if let Some(value) = self {
			value.to_query()
		} else {
			"".to_owned()
		}
	}
}
