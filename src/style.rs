use html_color::*;
use serde::Serialize;

#[derive(Debug, PartialEq, FromFormField, Serialize)]
pub enum Theme {
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
pub struct Style {
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
