#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;
use html_color::*;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, PartialEq, FromFormField, Serialize)]
enum Theme {
	Light,
	Dark,
	Black,
}

impl Default for Theme {
	fn default() -> Self {
		Theme::Light
	}
}

#[derive(Debug, PartialEq, Serialize)]
struct Style {
	font_color: &'static str,
	background_color: &'static str,
}

impl From<Theme> for Style {
	fn from(theme: Theme) -> Self {
		match theme {
			Theme::Light => Style {
				font_color: BLACK,
				background_color: WHITE,
			},
			Theme::Dark => Style {
				font_color: WHITE,
				background_color: "#22252d",
			},
			Theme::Black => Style {
				font_color: WHITE,
				background_color: BLACK,
			},
		}
	}
}

#[get("/?<theme>")]
fn index(theme: Option<Theme>) -> Template {
	let style: Style = theme.unwrap_or_default().into();
	Template::render(
		"index",
		context! {cargo_pkg_version: CARGO_PKG_VERSION, cargo_pkg_name: CARGO_PKG_NAME, style},
	)
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index]).attach(Template::fairing())
}
