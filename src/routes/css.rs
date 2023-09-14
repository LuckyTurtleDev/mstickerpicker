use poem::{handler, Response};

#[handler]
pub async fn css() -> Response {
	let css = include_str!("../components/style.css");
	Response::from(css).set_content_type("text/css")
}
