use crate::{
	error::ServerError,
	html::Html,
	style::{Style, Theme},
	tera::render_template,
	CARGO_PKG_NAME, CARGO_PKG_VERSION,
};
use ::tera::Context;
use actix_web::{get, web};
use actix_web_lab::extract::Query;
use anyhow::Result;
use serde::{self, de, Deserialize};

fn deserilize_theme<'de, D>(deserializer: D) -> Result<Theme, D::Error>
where
	D: de::Deserializer<'de>,
{
	let theme_vec: Vec<Theme> = Vec::deserialize(deserializer)?;
	Ok(theme_vec.into_iter().next().unwrap_or_default())
}

#[derive(Debug, Deserialize)]
pub struct IndexQuerry {
	#[serde(default)]
	#[serde(deserialize_with = "deserilize_theme")]
	theme: Theme,
	user: Option<String>,
}

#[get("/")]
pub async fn index(query: Query<IndexQuerry>) -> Result<Html, ServerError> {
	let style: Style = query.theme.into();
	let mut context = Context::new();
	context.insert("cargo_pkg_version", CARGO_PKG_VERSION);
	context.insert("cargo_pkg_name", CARGO_PKG_NAME);
	context.insert("style", &style);
	Ok(render_template("index.html", &context)?.into())
}
