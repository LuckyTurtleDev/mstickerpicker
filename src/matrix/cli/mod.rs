mod import;

use clap::Parser;
use log::error;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, UserId};
use once_cell::sync::Lazy;

#[derive(Parser, Debug)]
#[command(bin_name = "")]
#[command(no_binary_name = true)]
enum SubCommands {
	Import(import::Opt)
}

pub static HELP: Lazy<String> = Lazy::new(|| {
	let err = SubCommands::try_parse_from(["--help"]).unwrap_err();
	format!("{err}")
});

/// store the matrix context of the message
struct Context<'a> {
	user: &'a UserId
}

pub async fn execute_cli(
	bot_user: &UserId,
	input: &str,
	user: &UserId
) -> RoomMessageEventContent {
	// TODO: deal with spaces in arguments
	// allow message to start with and with out "botname: "
	let input = input
		.trim()
		.trim_start_matches(bot_user.localpart())
		.trim_start_matches(':')
		.trim_start()
		.split(' ');
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
	let context = Context { user };
	let res = match command {
		SubCommands::Import(opt) => import::run(opt, context).await
	};
	match res {
		Ok(value) => value,
		Err(err) => {
			error!("{err}");
			err.into()
		}
	}
}
