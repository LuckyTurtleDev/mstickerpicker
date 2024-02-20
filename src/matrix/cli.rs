use crate::CARGO_PKG_NAME;
use clap::{Command, Parser, Subcommand as _};
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, UserId};

#[derive(Parser, Debug)]
enum SubCommands {
	Foo,
	Baa
}

pub fn execute_cli(
	bot_user: &UserId,
	input: &str
) -> anyhow::Result<RoomMessageEventContent> {
	let cli = Command::new(CARGO_PKG_NAME)
    .bin_name("") //bin name ist still shown afer "Usage:"
    .no_binary_name(true);
	let cli = SubCommands::augment_subcommands(cli);
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
	Ok(RoomMessageEventContent::text_plain("Sucess"))
}
