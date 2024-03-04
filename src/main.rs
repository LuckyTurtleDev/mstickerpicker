mod components;
mod error;
mod matrix;
mod routes;
mod sql;
mod style;

use anyhow::Context;
use dotenv::dotenv;
use error::*;
use log::info;
use matrix::{start_matrix, MatrixConfig};
use once_cell::sync::Lazy;
use routes::get_router;
use serde::{de, Deserialize};
use tokio::try_join;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

static CONFIG: Lazy<Config> = Lazy::new(|| {
	let matrix = MatrixConfig::from_env();
	Config { matrix }
});

static SQL_POOL: Lazy<sqlx::Pool<sqlx::Postgres>> = Lazy::new(|| {
	tokio::runtime::Runtime::new().unwrap().block_on(async {
		let pool = sqlx::PgPool::connect("postgres://localhost/mstickerpicker")
			.await
			.context("can not connect to database")
			.ok_or_exit();
		sqlx::migrate!("src/migrations")
			.run(&pool)
			.await
			.context("database migration has failed")
			.ok_or_exit();
		pool
	})
});

///read only config
struct Config {
	matrix: MatrixConfig
}

fn load_env(var: &str) -> String {
	std::env::var(var)
		.with_context(|| format!("Enviroment variable {var:?} must be set"))
		.ok_or_exit()
}

fn main() {
	dotenv().ok();
	Lazy::force(&CONFIG);
	my_env_logger_style::builder()
		//the filter do not work for some reason
		.filter_module("matrix_sdk", log::LevelFilter::Warn)
		.filter_module("matrix_sdk_base", log::LevelFilter::Warn)
		.filter_module("matrix_sdk_crypto", log::LevelFilter::Warn)
		.filter_module("ruma_common", log::LevelFilter::Warn)
		.init();
	info!("starting {CARGO_PKG_NAME} v{CARGO_PKG_VERSION}");
	Lazy::force(&SQL_POOL);
	tokio_main().ok_or_exit();
}

#[tokio::main]
async fn tokio_main() -> anyhow::Result<()> {
	let router = get_router();
	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
		.await
		.context("failed to bind socket")?;
	info!("statring web server at http://localhost:8080/");
	try_join!(
		async {
			axum::serve(listener, router)
				.await
				.context("failed to start axum webserver")
		},
		async {
			start_matrix()
				.await
				.context("error while running matrix client")
		}
	)?;
	Ok(())
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
