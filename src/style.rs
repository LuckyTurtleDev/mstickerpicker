use html_color::*;
use serde::{Deserialize, Serialize};

const ELEMENT_GREEN: &'static str = "#0dbd8b";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
	#[default]
	Light,
	Dark,
	Black,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Style {
	font_color: &'static str,
	font_pale_color: &'static str,
	background_color: &'static str,
	accent_color: &'static str,
}

impl From<Theme> for Style {
	fn from(theme: Theme) -> Self {
		match theme {
			Theme::Light => Style {
				font_color: BLACK,
				font_pale_color: GREY,
				background_color: WHITE,
				accent_color: ELEMENT_GREEN,
			},
			Theme::Dark => Style {
				font_color: WHITE,
				font_pale_color: GREY,
				background_color: "#21262c",
				accent_color: ELEMENT_GREEN,
			},
			Theme::Black => Style {
				font_color: WHITE,
				font_pale_color: GREY,
				background_color: BLACK,
				accent_color: ELEMENT_GREEN,
			},
		}
	}
}
