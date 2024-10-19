use log::error;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
enum InnerError {
	#[error("Database::Error")]
	Disconnect(#[from] sqlx::Error)
}

#[derive(Debug)]
pub struct MyError {
	error: InnerError,
	context: Vec<String>
}

impl<E> From<E> for MyError
where
	E: Into<InnerError>
{
	fn from(value: E) -> Self {
		Self {
			error: value.into(),
			context: vec![]
		}
	}
}

impl Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// build anyhow like error message
		//foo
		//
		//Caused by:
		//    0: baa
		//    1: buu
		let mut iter = self.context.iter();
		if let Some(c1) = iter.next() {
			write!(f, "{c1}\n\nCaused by:\n    ")?;
			for (i, c) in iter.enumerate() {
				write!(f, "{i}:{c}\n    ")?;
			}
			// numbers are only shown if theire exist more than one context
			if self.context.len() >= 2 {
				write!(f, "{}:", self.context.len() - 1)?;
			}
		}
		writeln!(f, "{}", self.error)
	}
}

impl MyError {
	pub fn log(&self) {
		error!("{}", self.error);
	}

	/// indicates if the error message should be shown to the enduser
	fn show_user(&self) -> bool {
		match self.error {
			_ => false
		}
	}

	/// create a message, which can be savely shown to the enduser
	fn user_message(&self) -> String {
		if self.show_user() {
			format!("{self}")
		} else {
			"Internal Server Error".to_owned()
		}
	}

	/// create a matrix event, which can be send to the matrix user
	pub fn matrix_event(&self) -> RoomMessageEventContent {
		RoomMessageEventContent::text_plain(self.user_message())
	}
}

/// add more context to the error. Insperated by [anyhow::Context]
pub trait MyContext<T> {
	/// Wrap the error value with additional context.
	fn my_context<C>(self, context: C) -> Result<T, MyError>
	where
		C: Display + Send + Sync + 'static;

	/// Wrap the error value with additional context that is evaluated lazily
	/// only once an error does occur.
	fn with_my_context<C, F>(self, f: F) -> Result<T, MyError>
	where
		C: Display + Send + Sync + 'static,
		F: FnOnce() -> C;
}

impl<T, E> MyContext<T> for Result<T, E>
where
	E: Into<MyError>
{
	fn my_context<C>(self, context: C) -> Result<T, MyError>
	where
		C: Display + Send + Sync + 'static
	{
		self.map_err(|err| {
			let mut err: MyError = err.into();
			err.context.push(format!("{context}"));
			err
		})
	}

	fn with_my_context<C, F>(self, f: F) -> Result<T, MyError>
	where
		C: Display + Send + Sync + 'static,
		F: FnOnce() -> C
	{
		self.map_err(|err| {
			let mut err: MyError = err.into();
			err.context.push(format!("{}", f()));
			err
		})
	}
}
