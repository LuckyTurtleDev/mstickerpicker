use dioxus::prelude::*;


#[allow(non_snake_case)]
#[component]
pub fn Edit(token: String) -> Element {
	use dioxus_logger::tracing::info;
	let mut packs = use_signal(|| "".to_owned());
	rsx! {
		div { display: "flex", flex_direction: "column",
			div { display: "flex", flex_direction: "column",
				"import Stickerpack from Telegram"
				input {
					value: packs,
					placeholder: "pack_id or pack_url",
					oninput: move |event| packs.set(event.value().trim_start().to_owned())
				}
				button {
					onclick: move |_| {
					    let token_c = token.clone();
					    async move {
					        let pack_list: Vec<String> = packs
					            .read()
					            .split(' ')
					            .filter(|f| !f.is_empty())
					            .map(|f| f.to_owned())
					            .collect();
					        info!("import telegram packs: {:?}", pack_list);
					        if !pack_list.is_empty() {
					            let _req = crate::server_fn::import_telegram_packs(token_c, pack_list)
					                .await
					                .ok();
					        }
					        packs.set("".to_owned());
					    }
					},
					disabled: packs.read().is_empty(),
					"import"
				}
			}
		}
	}
}
