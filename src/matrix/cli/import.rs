use super::Context;
use crate::Error;
use clap::Parser;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use mstickerlib::tg::pack_url_to_name;

#[derive(Debug, Parser)]
pub struct Opt {
	/// Pack url
	#[clap(required = true)]
	packs: Vec<String>
}

pub async fn run<'a>(
	opt: Opt,
	_context: Context<'a>
) -> Result<RoomMessageEventContent, Error> {
	Ok(RoomMessageEventContent::text_plain("TODO"))
}
