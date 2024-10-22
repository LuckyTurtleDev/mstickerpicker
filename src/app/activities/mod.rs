mod home;
mod user_home;
mod edit;

use dioxus::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use home::Home;
use user_home::UserHome;
use edit::Edit;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},
    #[route("/token/:token")]
    UserHome { token: String},
    #[route("/edit/:token")]
    Edit { token: String}
}
