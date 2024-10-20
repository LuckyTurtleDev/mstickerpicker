use dioxus::prelude::*;


#[allow(non_snake_case)]
#[component]
pub fn UserHome(token: String) -> Element {
    rsx!(
        {format!("your token is {token:?}")}
    )
}