use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, UserId};
use once_cell::sync::Lazy;

pub static HELP: Lazy<String> = Lazy::new(|| {
	let err = SubCommands::try_parse_from(["--help"]).unwrap_err();
	format!("{err}")
});

#[derive(Parser, Debug)]
#[command(bin_name = "")]
#[command(no_binary_name = true)]
enum SubCommands {
	WebLogin
}

pub async fn execute_cli(bot_user: &UserId, input: &str) -> RoomMessageEventContent {
	// allow message to also start with and with out "botname: "
	let input = input
		.trim()
		.trim_start_matches(bot_user.localpart())
		.trim_start_matches(':')
		.trim_start()
		.split(' ')
		.filter(|f| !f.is_empty());
	let command = match SubCommands::try_parse_from(input) {
		Ok(value) => value,
		Err(err) => {
			// avoid getting "error:" infront of the response. (especially if the user has call --help)
			// We also not need to log this
			return RoomMessageEventContent::text_plain(
				format!("{err}").trim_start_matches("error: ")
			);
		}
	};

	RoomMessageEventContent::text_plain("todo")
}
