use crate::{
	style::{Style, Theme},
	CARGO_PKG_NAME,
};
use yew::ServerRenderer;

pub mod input;

pub(crate) async fn render<T>(theme: Option<Theme>) -> poem::web::Html<String>
where
	T: yew::html::BaseComponent,
	T::Properties: Default,
{
	let renderer = ServerRenderer::<T>::new();
	let body = renderer.render().await;
	let mut style = format!(":root {{{}}}", Style::from(theme.unwrap_or_default()).to_css());
	if theme.is_none() {
		style += &format!(
			"\n@media (prefers-color-scheme: dark) {{:root {{{}}}}}",
			Style::from(Theme::Dark).to_css()
		)
	}
	poem::web::Html(format!(
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
