mod activities;

use dioxus::prelude::*;
use activities::Route;


#[cfg(feature = "web")]
pub fn main_app() {
	use dioxus_logger::tracing::Level;

	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");

	// Hydrate the application on the client
	dioxus::web::launch::launch_cfg(App, dioxus::web::Config::new().hydrate(true));
}

#[allow(non_snake_case)]
pub fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}
