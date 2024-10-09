use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
	#[route("/")]
	Home {}
}

pub fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}

#[component]
fn Home() -> Element {
	rsx! {
		div {
			//color: "green",
			"Hello world"
		}
	}
}
