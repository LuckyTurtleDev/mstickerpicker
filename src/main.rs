#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
use anyhow::Result;
use dotenv::dotenv;
use futures_util::future::join_all;
use mstickereditor::stickerpicker::StickerPack;
use once_cell::sync::Lazy;
use s3::{Bucket, Region};
use serde::Deserialize;
use std::{env, process::exit};
mod style;
use ::tera::Context;
use actix_web::{
	get,
	http::{header::ContentType, StatusCode},
	middleware::Logger,
	web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use style::{Style, Theme};

mod tera;
use crate::tera::render_template;

mod error;
use error::ServerError;

mod html;
use html::Html;

mod routes;

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
static WIDGET_API: Lazy<String> = Lazy::new(|| {
	include_str!("js/widget-api.js")
		.replace("export", "")
		.replace("sendSticker", "widgetAPISendSticker")
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

/*
async fn stickerpicker(user: &str, style: &Style) -> Result<Template> {
	{
		let mut file_paths = BUCKET
			.list(format!("/{}/", user), Some("/".to_owned()))
			.await
			.context("Error listing bucket:")?
			.into_iter()
			.flat_map(|chunk| chunk.contents.into_iter())
			.map(|obj| obj.key)
			.filter(|key| key.ends_with(".json"))
			.collect::<Vec<_>>();
		file_paths.sort_unstable();
		let files = file_paths.into_iter().map(|path| BUCKET.get_object(path));
		let files = join_all(files).await.into_iter();
		let mut packs: Vec<StickerPack> = Vec::with_capacity(files.len());
		for file in files {
			match file {
				Err(err) => error!("Error loading Stickerpack from bucket {err}"),
				Ok(value) => {
					let result: Result<StickerPack, _> = serde_json::from_slice(value.bytes());
					match result {
						Err(err) => error!("Error parsing Stickerpack {err}"),
						Ok(value) => packs.push(value),
					}
				},
			}
		}

		Ok(Template::render(
			"picker",
			context! {cargo_pkg_name: CARGO_PKG_NAME, packs, style, widget_api: &*WIDGET_API},
		))
	}
}*/

#[actix_web::main]
async fn actix_main() -> std::io::Result<()> {
	BUCKET
		.list("/".to_owned(), Some("/".to_owned()))
		.await
		.expect("failed to connect to s3 bucket");
	HttpServer::new(|| {
		App::new()
			.service(routes::index::index)
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
