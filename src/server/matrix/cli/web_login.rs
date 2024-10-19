use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;

use super::CliContext;
use crate::server::{config, error::MyError, sql};

pub async fn web_login(
	context: CliContext<'_>
) -> Result<RoomMessageEventContent, MyError> {
	let token = sql::add_new_token(context.user_id).await?;
	let base_url = &config().base_url;
	Ok(RoomMessageEventContent::text_plain(format!(
		"Login successful: {base_url}/token/{token}"
	)))
}
