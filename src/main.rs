mod app;
#[cfg(feature = "server")]
mod server;
mod server_fn;

fn main() {
	#[cfg(feature = "web")]
	app::main_app();

	#[cfg(feature = "server")]
	server::main_server();
}
