use axum::response::Response;
use http::header::CONTENT_TYPE;

pub async fn css() -> Response<String> {
	// hot reload style at debug mode
	#[cfg(debug_assertions)]
	let css = tokio::fs::read_to_string(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/src/components/style.css"
	))
	.await
	.unwrap();
	#[cfg(not(debug_assertions))]
	let css = include_str!(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/src/components/style.css"
	))
	.to_owned();
	Response::builder()
		.header(CONTENT_TYPE, "text/css")
		.body(css)
		.unwrap()
}
