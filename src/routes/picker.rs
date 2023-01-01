use crate::{
	error::ServerError,
	html::Html,
	style::{deserilize_theme, Style, Theme},
	tera::render_template,
	BUCKET, CARGO_PKG_NAME,
};
use ::tera::Context;
use actix_web::get;
use actix_web_lab::extract::Query;
use anyhow::Result;
use futures_util::future::join_all;
use log::error;
use mstickereditor::stickerpicker::StickerPack;
use once_cell::sync::Lazy;
use serde::{self, Deserialize};

static WIDGET_API: Lazy<String> = Lazy::new(|| {
	include_str!("../js/widget-api.js")
		.replace("export", "")
		.replace("sendSticker", "widgetAPISendSticker")
});

#[derive(Debug, Deserialize)]
pub struct QueryData {
	#[serde(default)]
	#[serde(deserialize_with = "deserilize_theme")]
	theme: Theme,
	user: String,
}

#[get("/picker")]
async fn picker(query: Query<QueryData>) -> Result<Html, ServerError> {
	let mut file_paths = BUCKET
		.list(format!("/{}/", query.user), Some("/".to_owned()))
		.await?
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

	let style: Style = query.theme.into();
	let mut context = Context::new();
	context.insert("cargo_pkg_name", CARGO_PKG_NAME);
	context.insert("packs", &packs);
	context.insert("style", &style);
	context.insert("widget_api", &*WIDGET_API);
	Ok(render_template("picker.html", &context)?.into())
}
