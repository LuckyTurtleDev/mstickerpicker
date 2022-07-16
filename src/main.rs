#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{context, Template};

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[get("/")]
fn index() -> Template {
	Template::render(
		"index",
		context! {cargo_pkg_version: CARGO_PKG_VERSION, cargo_pkg_name: CARGO_PKG_NAME},
	)
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index]).attach(Template::fairing())
}
