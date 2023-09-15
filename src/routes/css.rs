use poem::{handler, Response};

#[handler]
pub async fn css() -> Response {
	// hot reload style at debug mode
	#[cfg(debug_assertions)]
	let css = tokio::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/components/style.css"))
		.await
		.unwrap();
	#[cfg(not(debug_assertions))]
	let css = include_str!("../components/style.css");
	Response::from(css).set_content_type("text/css")
}
