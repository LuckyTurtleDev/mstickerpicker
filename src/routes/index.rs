use crate::{
	error::ServerError,
	html::Html,
	style::{deserilize_theme, Style, Theme},
	tera::render_template,
	CARGO_PKG_NAME, CARGO_PKG_VERSION,
};
use ::tera::Context;
use actix_web::get;
use actix_web_lab::extract::Query;
use anyhow::Result;
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub struct QueryData {
	#[serde(default)]
	#[serde(deserialize_with = "deserilize_theme")]
	theme: Theme,
	user: Option<String>,
}

#[get("/")]
pub async fn index(query: Query<QueryData>) -> Result<Html, ServerError> {
	let style: Style = query.theme.into();
	let mut context = Context::new();
	context.insert("cargo_pkg_version", CARGO_PKG_VERSION);
	context.insert("cargo_pkg_name", CARGO_PKG_NAME);
	context.insert("style", &style);
	Ok(render_template("index.html", &context)?.into())
}
