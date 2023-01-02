use actix_web::{
	http::{header::ContentType, StatusCode},
	HttpResponse,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
	#[error("{0}")]
	Tera(#[from] tera::Error),
	#[error("{0}")]
	S3Error(#[from] s3::error::S3Error),
	#[error("User has used wrong token")]
	WrongToken,
}

impl actix_web::error::ResponseError for ServerError {
	fn error_response(&self) -> HttpResponse {
		let message = match self {
			Self::WrongToken => "wrong token",
			_ => "",
		};
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::plaintext())
			.body(format!("{} {message} \n", self.status_code().to_string()))
	}
	fn status_code(&self) -> StatusCode {
		match *self {
			Self::WrongToken => StatusCode::UNAUTHORIZED,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}
