use crate::{components::render, style::Theme, take_first};
use axum::response::Html;
use axum_extra::extract::Query;

use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize)]
pub struct QueryData {
	#[serde(default, deserialize_with = "take_first")]
	theme: Option<Theme>
}

pub async fn login(query: Query<QueryData>) -> Html<String> {
	render::<App>(query.0.theme).await
}

#[function_component]
fn App() -> yew::Html {
	html! {
		<div class="center">
		{"TODO"}
	</div>
	}
}
