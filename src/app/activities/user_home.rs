use dioxus::prelude::*;
use crate::app::activities::Route;


#[allow(non_snake_case)]
#[component]
pub fn UserHome(token: String) -> Element {
    let token_copy = token.clone();
    let valid  = use_server_future( move || crate::server_fn::check_token(token_copy.clone()))?;
    rsx!(
		"your token is {token:?} and is valid {valid.value():?}"
		Link { to: Route::Edit { token },
			button { "edit Stickerpacks" }
		}
		button { "enable Picker" }
	)
    
}