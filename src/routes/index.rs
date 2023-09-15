use crate::{
	components::render,
	style::{Style, Theme},
	CARGO_PKG_NAME, CARGO_PKG_VERSION,
};
use poem::{
	handler,
	web::{Html, Query},
};
use serde::{self, Deserialize};
use yew::prelude::*;
use crate::components::input::Input;

#[derive(Debug, Deserialize)]
pub struct QueryData {
	theme: Option<Theme>,
	user: Option<String>,
}

#[handler]
pub async fn index(Query(query): Query<QueryData>) -> Html<String> {
	render::<App>(query.theme).await
}

#[derive(PartialEq, Properties)]
pub struct Props {}

#[function_component]
fn App() -> yew::Html {
	html! {
		<div class="center">
		<h1>{{CARGO_PKG_NAME}}</h1>
		<p></p>
		<p>{"Register"}</p>
		<div class="login">
			<Input id={"register_token"} label={"register token"}/>
			<button type="button">{"Register"}</button>
		</div>
		<p></p>
		<p> {CARGO_PKG_NAME}{" v "}{CARGO_PKG_VERSION}{" is running at this server"}</p>
	</div>
	}
}
