mod web_login;

use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, UserId};
use once_cell::sync::Lazy;

use crate::server::{error::MyContext, sql};

/// store help message of the cli
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

pub struct CliContext<'a> {
	#[allow(dead_code)]
	pub bot_user: &'a UserId,
	/// matrix id of the user, which has send the message
	#[allow(dead_code)]
	pub user: &'a UserId,
	/// database id of the user, which has send the message
	pub user_id: i32
}

pub async fn execute_cli(
	bot_user: &UserId,
	user: &UserId,
	user_id: Option<i32>,
	input: &str
) -> RoomMessageEventContent {
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
	// get user id if is still missing (we need it nearly for everything)
	let user_id = match user_id {
		Some(id) => id,
		None => {
			match sql::get_or_creat_user_id(user)
				.await
				.my_context("failed to look up or create user at the database")
			{
				Ok(id) => id,
				Err(err) => {
					err.log();
					return err.matrix_event();
				}
			}
		}
	};
	let context = CliContext {
		bot_user,
		user,
		user_id
	};
	let res = match command {
		SubCommands::WebLogin => web_login::web_login(context).await
	};
	match res {
		Ok(event) => event,
		Err(err) => {
			err.log();
			err.matrix_event()
		}
	}
}
