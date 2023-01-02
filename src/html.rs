use actix_web::{
	body::BoxBody,
	http::{header::ContentType, StatusCode},
	HttpRequest, HttpResponse, Responder,
};

pub struct Html {
	body: String,
}

impl From<String> for Html {
	fn from(value: String) -> Self {
		Html { body: value }
	}
}

impl Responder for Html {
	type Body = BoxBody;

	fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
		HttpResponse::build(StatusCode::OK)
			.insert_header(ContentType::html())
			.body(self.body)
	}
}
