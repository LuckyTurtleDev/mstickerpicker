use dioxus::prelude::*;

#[cfg(feature = "web")]
pub fn main_app() {
	use dioxus_logger::tracing::Level;

	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");

	// Hydrate the application on the client
	dioxus::web::launch::launch_cfg(App, dioxus::web::Config::new().hydrate(true));
}

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
