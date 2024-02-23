use crate::{sql::get_or_creat_user_id, CARGO_PKG_NAME};
use clap::{Command, Parser, Subcommand as _};
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, UserId};
use once_cell::sync::Lazy;

#[derive(Parser, Debug)]
enum SubCommands {
	Foo,
	Baa
}

pub fn get_command() -> Command {
	static COMMAND: Lazy<Command> = Lazy::new(|| {
		let cli = Command::new(CARGO_PKG_NAME)
    .bin_name("") //bin name ist still shown afer "Usage:"
    .no_binary_name(true);
		SubCommands::augment_subcommands(cli)
	});
	COMMAND.to_owned()
}

pub async fn execute_cli(
	bot_user: &UserId,
	input: &str,
	user: &UserId
) -> anyhow::Result<RoomMessageEventContent> {
	let cli = get_command();
	// TODO: deal with spaces in arguments
	// allow message to start with and with out "botname: "
	let input = input
		.trim()
		.trim_start_matches(bot_user.localpart())
		.trim_start_matches(':')
		.trim_start()
		.split(' ');
	let _ = match cli.try_get_matches_from(input) {
		Ok(value) => value,
		// avoid getting "error:" infront of the response. (especially if the user has call --help)
		// We also not need to log this
		Err(err) => {
			return Ok(RoomMessageEventContent::text_plain(
				format!("{err}").trim_start_matches("error: ")
			))
		},
	};
	let res = get_or_creat_user_id(user).await;
	Ok(RoomMessageEventContent::text_plain(format!("{res:?}")))
}
