mod cli;
mod join;
mod message;

use crate::{load_env, CONFIG};
use join::*;
use log::info;
use matrix_sdk::{
	config::SyncSettings,
	ruma::{OwnedServerName, OwnedUserId, ServerName, UserId},
	Client
};
use message::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct MatrixConfig {
	homeserver_url: String,
	username: String,
	password: String,
	user_allowed: UserAllowed
}

impl MatrixConfig {
	pub fn from_env() -> Self {
		Self {
			user_allowed: UserAllowed::from_env(),
			homeserver_url: load_env("MATRIX_HOMESERVER_URL"),
			username: load_env("MATRIX_USERNAME"),
			password: load_env("MATRIX_PASSWORD")
		}
	}
}
#[derive(Debug)]
enum UserAllowed {
	All,
	Some(HashSet<UserOrServer>)
}

impl UserAllowed {
	fn from_env() -> Self {
		#![allow(clippy::expect_fun_call)]
		let env = load_env("MATRIX_ALLOWED_USERS");
		if env == "all" {
			Self::All
		} else {
			let allowed_user = env.split(',').map(|f| {
				if f.contains(':') {
					let user = UserId::parse(f)
						.expect(&format!("{f:?} is no valid matrix user name"));
					UserOrServer::User(user)
				} else {
					let server = ServerName::parse(f)
						.expect(&format!("{f:?} is no valid matrix server name"));
					UserOrServer::Server(server)
				}
			});
			Self::Some(allowed_user.collect())
		}
	}

	fn is_allowed(&self, user: &OwnedUserId) -> bool {
		match self {
			Self::All => true,
			Self::Some(set) => {
				let server = user.server_name();
				set.contains(&UserOrServer::Server(server.into()))
					|| set.contains(&UserOrServer::User(user.to_owned()))
			}
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum UserOrServer {
	Server(OwnedServerName),
	User(OwnedUserId)
}

pub async fn start_matrix() -> anyhow::Result<()> {
	// Note that when encryption is enabled, you should use a persistent store to be
	// able to restore the session with a working encryption setup.
	// See the `persist_session` example.
	let client = Client::builder()
		.homeserver_url(&CONFIG.matrix.homeserver_url)
		.build()
		.await?;
	client
		.matrix_auth()
		.login_username(&CONFIG.matrix.username, &CONFIG.matrix.password)
		.initial_device_display_name("command bot")
		.await?;

	info!("logged in at matrix as {}", CONFIG.matrix.username);

	// An initial sync to set up state and so our bot doesn't respond to old
	// messages. But we want to still process invites.
	client.add_event_handler(on_join);
	let response = client.sync_once(SyncSettings::default()).await?;
	// add our CommandBot to be notified of incoming messages, we do this after the
	// initial sync to avoid responding to messages before the bot was running.
	client.add_event_handler(on_room_message);

	// since we called `sync_once` before we entered our sync loop we must pass
	// that sync token to `sync`
	let settings = SyncSettings::default().token(response.next_batch);
	// this keeps state from the server streaming in to CommandBot via the
	// EventHandler trait
	client.sync(settings).await?;

	Ok(())
}