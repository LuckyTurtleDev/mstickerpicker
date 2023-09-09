use crate::{
	style::{deserilize_theme, Style, Theme},
	CARGO_PKG_NAME, CARGO_PKG_VERSION,
};
use ::tera::Context;
use anyhow::Result;
use poem::{
	handler,
	web::{Html, Query},
};
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub struct QueryData {
	#[serde(default)]
	theme: Theme,
	user: Option<String>,
}

#[handler]
pub fn index(Query(query): Query<QueryData>) -> Result<Html<String>, poem::Error> {
	let style: Style = query.theme.into();
	let mut context = Context::new();
	context.insert("cargo_pkg_version", CARGO_PKG_VERSION);
	context.insert("cargo_pkg_name", CARGO_PKG_NAME);
	context.insert("style", &style);
	Ok(Html("hello world".to_owned()))
}
