use actix_web::{http::header::ContentType, HttpResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
	#[error("{0}")]
	Tera(#[from] tera::Error),
}

impl actix_web::error::ResponseError for ServerError {
	fn error_response(&self) -> HttpResponse {
		let message = match self {
			_ => "",
		};
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::plaintext())
			.body(format!("{} \n{message}", self.status_code().to_string()))
	}
}
