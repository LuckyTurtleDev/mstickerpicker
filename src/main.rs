#![allow(non_snake_case)]

mod app;
#[cfg(feature = "server")]
mod server;

fn main() {
	#[cfg(feature = "web")]
	app::main_app();

	#[cfg(feature = "server")]
	server::main_server();
}
