#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

use anyhow::Result;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use s3::{Bucket, Region};
use serde::Deserialize;
use std::{env, process::exit};

use actix_web::{middleware::Logger, App, HttpServer};

mod error;
mod html;
mod routes;
mod style;
mod tera;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Config {
	#[serde(rename = "PACKS_S3_SERVER")]
	s3_server: String,
	#[serde(rename = "PACKS_S3_BUCKET")]
	s3_bucket: String,
	register_token: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
	dotenv().ok();
	let config: Result<Config, _> = de_env::from_env();
	config.unwrap_or_else(|err| {
		eprintln!("error loading Environment Variable:\n {:?}", err);
		exit(1)
	})
});

static BUCKET: Lazy<Bucket> = Lazy::new(|| {
	let region = Region::Custom {
		region: CONFIG.s3_server.clone(),
		endpoint: CONFIG.s3_server.clone(),
	};
	Bucket::new_public(&CONFIG.s3_bucket, region)
		.expect("Failed to open bucket")
		.with_path_style()
});

static SQL_POOL: Lazy<sqlx::Pool<sqlx::Postgres>> = Lazy::new(|| {
	tokio::runtime::Runtime::new().unwrap().block_on(async {
		let pool = sqlx::PgPool::connect("postgres://localhost/mstickerpicker")
			.await
			.expect("can not connect to database");
		sqlx::migrate!("src/migrations")
			.run(&pool)
			.await
			.expect("database migration has failed");
		pool
	})
});

#[actix_web::main]
async fn actix_main() -> std::io::Result<()> {
	BUCKET
		.list("/".to_owned(), Some("/".to_owned()))
		.await
		.expect("failed to connect to s3 bucket");
	HttpServer::new(|| {
		App::new()
			.service(routes::index::index)
			.service(routes::picker::picker)
			.service(routes::register::register)
			.wrap(Logger::new("%U by %{User-Agent}i -> %s in %T second"))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}

fn main() {
	env_logger::init();
	Lazy::force(&CONFIG);
	Lazy::force(&SQL_POOL);
	actix_main().expect("failed to start web server");
}
