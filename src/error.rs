use http::uri::InvalidUri;
use log::error;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use std::{fmt::Debug, process::exit};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Database error: `{0}`")]
	Database(#[from] sqlx::Error),
	#[error(transparent)]
	MStickerLib(#[from] mstickerlib::error::Error)
}

impl Error {
	fn show_user(&self) -> bool {
		match self {
			Self::Database(_) => false,
			Self::MStickerLib(mstickerlib::error::Error::InvalidPackUrl(_)) => true,
			Self::MStickerLib(mstickerlib::error::Error::InvalidHomeServerUrl(_)) => true,
			Self::MStickerLib(mstickerlib::error::Error::GifDecoding(_)) => true,
			Self::MStickerLib(mstickerlib::error::Error::GifEncoding(_)) => true,
			Self::MStickerLib(mstickerlib::error::Error::Ffmpeg(_)) => true,
			Self::MStickerLib(mstickerlib::error::Error::Webp(_)) => true,
			Self::MStickerLib(mstickerlib::error::Error::AnimationLoadError) => true,
			Self::MStickerLib(mstickerlib::error::Error::NoMimeType(_)) => true,
			Self::MStickerLib(_) => false
		}
	}
}

impl Into<RoomMessageEventContent> for &Error {
	fn into(self) -> RoomMessageEventContent {
		if self.show_user() {
			return RoomMessageEventContent::text_plain(format!("{self}"));
		}
		RoomMessageEventContent::text_plain("Internal Server Error")
	}
}
impl Into<RoomMessageEventContent> for Error {
	fn into(self) -> RoomMessageEventContent {
		(&self).into()
	}
}

pub trait ErrorFun {
	type Item;
	/// eprint Debug and exit if Result is error.
	/// Otherwise return value
	fn ok_or_exit(self) -> Self::Item;
	fn ok_or_log(self) -> Option<Self::Item>;
}

impl<T, E> ErrorFun for Result<T, E>
where
	E: Debug
{
	type Item = T;
	fn ok_or_exit(self) -> Self::Item {
		match self {
			Ok(value) => value,
			Err(err) => {
				eprintln!("\nError: {err:?}");
				exit(1);
			}
		}
	}

	fn ok_or_log(self) -> Option<Self::Item> {
		match self {
			Ok(value) => Some(value),
			Err(err) => {
				error!("Error: {err:?}");
				None
			}
		}
	}
}
