use axum::{
	routing::{get, post},
	Router
};
use log::info;
use serde::{de, Deserialize};

mod components;
mod routes;
mod style;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
	println!("Hello, world!");
	my_env_logger_style::just_log();
	let app = Router::new()
		.route("/", get(routes::index))
		.route("/css", get(routes::css))
		.route("/register", get(routes::register))
		.route("/register", post(routes::register_post))
		.route("/login", get(routes::login));
	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
	info!("statring server at http://localhost:8080/");
	axum::serve(listener, app).await.unwrap();
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
