#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
mod style;
use anyhow::Result;
use colored::*;
use once_cell::sync::Lazy;
use s3::{Bucket, Region};
use std::env;
use style::{Style, Theme};

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

static BUCKET: Lazy<Bucket> = Lazy::new(|| {
	let s3_server = env::var("PACKS_S3_SERVER").expect("PACKS_S3_SERVER must be set");
	let s3_bucket = env::var("PACKS_S3_BUCKET").expect("PACKS_S3_BUCKET must be set");
	let region = Region::Custom {
		region: s3_server.clone(),
		endpoint: s3_server,
	};
	Bucket::new_public(&s3_bucket, region)
		.expect("Failed to open bucket")
		.with_path_style()
});

pub trait ToResultStatus<T> {
	fn to_res_stat(self) -> Result<T, Status>;
}

impl<T> ToResultStatus<T> for anyhow::Result<T> {
	fn to_res_stat(self) -> Result<T, Status> {
		match self {
			Ok(value) => Ok(value),
			Err(err) => {
				eprintln!("   {} {}", ">>".bold(), format!("{} {}", "Error:".bold(), err).red());
				Err(Status::InternalServerError)
			},
		}
	}
}

#[get("/?<theme>&<user>")]
async fn index(theme: Option<Theme>, user: Option<&str>) -> Result<Template, Status> {
	let style: Style = theme.unwrap_or_default().into();
	match user {
		None => Ok(Template::render(
			"index",
			context! {cargo_pkg_version: CARGO_PKG_VERSION, cargo_pkg_name: CARGO_PKG_NAME, style},
		)),
		Some(user) => stickerpicker(user, &style).await.to_res_stat(),
	}
}

async fn stickerpicker(user: &str, style: &Style) -> Result<Template> {
	{
		BUCKET.list(format!("/{}/", user), Some("/".to_owned())).await?;
		Ok(Template::render(
			"picker",
			context! {cargo_pkg_version: CARGO_PKG_VERSION, cargo_pkg_name: CARGO_PKG_NAME, style},
		))
	}
}

#[launch]
fn rocket() -> _ {
	_ = BUCKET.list("/".to_owned(), Some("/".to_owned())); //init lazy
	rocket::build().mount("/", routes![index]).attach(Template::fairing())
}
