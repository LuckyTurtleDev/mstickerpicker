use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub id: &'static str,
    pub label: &'static str,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    html!{ <div class="floating_input">
				<input type="text" id={props.id} required=true />
				<label for={props.id}>{props.label}</label>
			</div>}
}