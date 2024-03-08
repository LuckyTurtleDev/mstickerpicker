use super::Context;
use crate::{Error, CONFIG};
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, UserId};
use mstickerlib::{database::Database, image::AnimationFormat, tg, tg::pack_url_to_name};

#[derive(Debug, Parser)]
pub struct Opt {
	/// Pack url
	#[clap(required = true)]
	packs: Vec<String>
}

/// avoid uploading the same file multiple times to matrix
struct DuplicateChecker<'a>(&'a UserId);
impl Database for DuplicateChecker<'_> {
	async fn add(
		&self,
		hash: mstickerlib::database::Hash,
		url: String
	) -> anyhow::Result<()> {
		todo!();
		Ok(())
	}

	async fn get(
		&self,
		hash: &mstickerlib::database::Hash
	) -> anyhow::Result<Option<String>> {
		todo!();
		Ok(None)
	}
}

pub async fn run(
	opt: Opt,
	context: Context<'_>
) -> Result<RoomMessageEventContent, Error> {
	//before doing anything, we check if the input is even a valid sticker url
	let mut pack_names = Vec::with_capacity(opt.packs.len());
	for pack in &opt.packs {
		pack_names.push(pack_url_to_name(pack).map_err(mstickerlib::error::Error::from)?);
	}
	for pack in pack_names {
		import_pack(pack, &context).await?;
	}
	Ok(RoomMessageEventContent::text_plain("finish task"))
}

async fn import_pack(pack: &str, context: &Context<'_>) -> Result<(), Error> {
	let tg_pack = tg::StickerPack::get(pack, &CONFIG.telegram).await?;
	let matrix_config = mstickerlib::matrix::Config {
		homeserver_url: context.client.homeserver().to_string(),
		user: context.bot_user.to_string(),
		access_token: context.client.access_token().unwrap()
	};
	let duplicate_checker = DuplicateChecker(context.user);
	let mut import_config = tg::ImportConfig::default();
	import_config.database = Some(&duplicate_checker);
	import_config.animation_format = AnimationFormat::Webp;
	let pack = tg_pack
		.import(&CONFIG.telegram, &matrix_config, &import_config)
		.await;
	Ok(())
}
