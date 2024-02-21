mod css;
mod index;
mod login;
mod register;

use axum::{
	routing::{get, post},
	Router
};
use css::css;
use index::index;
use login::login;
use register::{register, register_post};

pub fn get_router() -> Router {
	Router::new()
		.route("/", get(index))
		.route("/css", get(css))
		.route("/register", get(register))
		.route("/register", post(register_post))
		.route("/login", get(login))
}
