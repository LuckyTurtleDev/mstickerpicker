use html_color::*;
use serde::{self, de, Deserialize, Serialize};

const ELEMENT_GREEN: &'static str = "#0dbd8b";

#[derive(Clone, Copy, Debug, strum_macros::Display, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "snake_case")]
pub enum Theme {
	#[default]
	Light,
	Dark,
	Black,
}

pub fn deserilize_theme<'de, D>(deserializer: D) -> Result<Theme, D::Error>
where
	D: de::Deserializer<'de>,
{
	let theme_vec: Vec<Theme> = Vec::deserialize(deserializer)?;
	Ok(theme_vec.into_iter().next().unwrap_or_default())
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Style {
	pub font_color: &'static str,
	pub font_pale_color: &'static str,
	pub background_color: &'static str,
	pub accent_color: &'static str,
}

impl Style {
	pub fn to_css(&self) -> String {
		let Style {
			font_color,
			font_pale_color,
			background_color,
			accent_color,
		} = self;
		format!("--font_color: {font_color}; --font_pale_color: {font_pale_color}; --background_color: {background_color}; --accent_color: {accent_color};")
	}
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
