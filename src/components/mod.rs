use crate::{
	style::{Style, Theme},
	CARGO_PKG_NAME
};
use axum::response;
use yew::{html::BaseComponent, ServerRenderer};

pub mod input;

pub async fn render_probs<B>(
	theme: Option<Theme>,
	props: <B as BaseComponent>::Properties
) -> response::Html<String>
where
	B: BaseComponent,
	<B as yew::BaseComponent>::Properties: Send
{
	let renderer: ServerRenderer<B> = ServerRenderer::with_props(|| props);
	let body = renderer.render().await;
	add_frame(theme, body)
}

pub async fn render<T>(theme: Option<Theme>) -> response::Html<String>
where
	T: yew::html::BaseComponent,
	T::Properties: Default
{
	let renderer = ServerRenderer::<T>::new();
	let body = renderer.render().await;
	add_frame(theme, body)
}

fn add_frame(theme: Option<Theme>, body: String) -> response::Html<String> {
	let mut style = format!(
		":root {{{}}}",
		Style::from(theme.unwrap_or(Theme::Light)).to_css()
	);
	if theme.is_none() {
		style += &format!(
			"\n@media (prefers-color-scheme: dark) {{:root {{{}}}}}",
			Style::from(Theme::Dark).to_css()
		)
	}
	response::Html(format!(
		r#"<!DOCTYPE html>
<html>
<head>
	<title>{CARGO_PKG_NAME}</title>
	<meta name="viewport" content="width=device-width,initial-scale=1.0,user-scalable=no"/>
	<style>
		{style}
	</style>
	<link rel="stylesheet" href="/css">
</head>
<body>
	{body}
</body>
</html>"#
	))
}
