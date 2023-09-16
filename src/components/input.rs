use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
	/// id and name attribute input
	pub id: &'static str,
	pub label: &'static str,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
	html! { <div class="floating_input">
		<input type="text" name={props.id} id={props.id} required=true />
		<label for={props.id}>{props.label}</label>
	</div>}
}
