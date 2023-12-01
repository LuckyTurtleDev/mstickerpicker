use crate::{
	components::render_probs, style::Theme, take_first, ToQuery, CARGO_PKG_NAME,
	CARGO_PKG_VERSION
};
use axum::response::Html;
use axum_extra::extract::Query;

use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize)]
pub struct QueryData {
	#[serde(default, deserialize_with = "take_first")]
	theme: Option<Theme>
}

pub async fn index(query: Query<QueryData>) -> Html<String> {
	let query = query.0;
	render_probs::<App>(query.theme, AppProps { theme: query.theme }).await
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
	pub theme: Option<Theme>
}

#[function_component]
fn App(props: &AppProps) -> yew::Html {
	let query = props.theme.to_query();
	html! {
		<div class="center">
		<h1>{{CARGO_PKG_NAME}}</h1>
		<p></p>
		<a href={format!("/login?{query}")}>
			<button>{"login"}</button>
		</a>
		<p></p>
		<a href={format!("/register?{query}")}>
			<button>{"Register"}</button>
		</a>
		<p></p>
		<p></p>
		<p> {CARGO_PKG_NAME}{" v"}{CARGO_PKG_VERSION}{" is running at this server"}</p>
	</div>
	}
}
