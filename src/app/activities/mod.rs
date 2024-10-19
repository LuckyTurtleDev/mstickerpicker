mod home;
mod user_home;

use dioxus::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use home::Home;
use user_home::UserHome;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},
    #[route("/token/:token")]
    UserHome { token: String}
}