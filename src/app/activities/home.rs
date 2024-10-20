use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> Element {
	rsx! {
		div {
			"Hello please contact the bot, to login"
		}
	}
}