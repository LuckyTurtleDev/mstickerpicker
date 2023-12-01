use crate::{
	components::{input::Input, render},
	style::Theme,
	take_first
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

pub async fn register(query: Query<QueryData>) -> Html<String> {
	render::<App>(query.0.theme).await
}

#[function_component]
fn App() -> yew::Html {
	html! {
		<div class="center">
			<p>{"Register"}</p>
			<div class="login">
				<form method="post">
					<Input id={"register_token"} label={"register token"}/>
					<button type="submit">{"Register"}</button>
				</form>
			</div>
		</div>
	}
}

pub async fn register_post(query: Query<QueryData>) -> Html<String> {
	render::<AppPost>(query.0.theme).await
}

#[function_component]
fn AppPost() -> yew::Html {
	html! {
		<div class="center">
			{"TODO"}
		</div>
	}
}
