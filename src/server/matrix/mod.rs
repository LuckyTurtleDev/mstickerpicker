mod join;

use anyhow::Context;
use envconfig::Envconfig;
use join::on_join;
use log::info;
use matrix_sdk::{
	config::SyncSettings, ruma::{OwnedUserId, UserId}, Client, OwnedServerName, ServerName
};
use once_cell::sync::OnceCell;
use std::{collections::HashSet, str::FromStr};

//const  USER_ALLOWED: OnceCell<UserAllowed> = OnceCell::new();

#[derive(Debug, Envconfig)]
pub struct MatrixConfig {
	#[envconfig(from = "MATRIX_HOMESERVER_URL")]
	homeserver_url: String,
	#[envconfig(from = "MATRIX_USERNAME")]
	username: String,
	#[envconfig(from = "MATRIX_PASSWORD")]
	password: String,
	#[envconfig(from = "MATRIX_ALLOWED_USERS")]
	user_allowed: UserAllowed
}

#[derive(Debug)]
enum UserAllowed {
	All,
	/// set of user and server names which are accepted
	Some((HashSet<OwnedUserId>, HashSet<OwnedServerName>))
}

impl UserAllowed {
	fn allowed(&self, user: &UserId) -> bool {
		match self {
			Self::All => true,
			Self::Some((users, servers)) => {
				users.contains(user) || servers.contains(user.server_name())
			},
		}
	}
}

impl FromStr for UserAllowed {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut users: HashSet<OwnedUserId> = Default::default();
		let mut servers: HashSet<OwnedServerName> = Default::default();
		for f in s.split(',') {
			if f == "all" {
				return Ok(UserAllowed::All);
			}
			if f.contains(':') {
				let user = UserId::parse(f)
					.with_context(|| format!("{f:?} is no valid matrix user name"))?;
				users.insert(user);
			} else {
				let server = ServerName::parse(f)
					.with_context(|| format!("{f:?} is no valid matrix server name"))?;
				servers.insert(server);
			}
		}
		if users.is_empty() && servers.is_empty() {
			info!("registration of new users is dissable");
		}
		Ok(UserAllowed::Some((users, servers)))
	}
}

pub async fn start_matrix() -> anyhow::Result<()> {
	let matrix_config = MatrixConfig::init_from_env()?;
	info!("{matrix_config:?}");
    //USER_ALLOWED.set(matrix_config.user_allowed).unwrap();

	// Note that when encryption is enabled, you should use a persistent store to be
	// able to restore the session with a working encryption setup.
	// See the `persist_session` example.
	let client = Client::builder()
		.homeserver_url(&matrix_config.homeserver_url)
		.build()
		.await?;
	client
		.matrix_auth()
		.login_username(&matrix_config.username, &matrix_config.password)
		.initial_device_display_name("command bot")
		.await?;

	info!("logged in at matrix server {} as {}", &matrix_config.homeserver_url, &matrix_config.username);

	// An initial sync to set up state and so our bot doesn't respond to old
	// messages. But we want to still process invites.
	client.add_event_handler(on_join);

    let response = client.sync_once(SyncSettings::default()).await?;

    // add our CommandBot to be notified of incoming messages, we do this after the
	// initial sync to avoid responding to messages before the bot was running.
	//client.add_event_handler(on_room_message);

	// since we called `sync_once` before we entered our sync loop we must pass
	// that sync token to `sync`
	let settings = SyncSettings::default().token(response.next_batch);
	// this keeps state from the server streaming in to CommandBot via the
	// EventHandler trait
	client.sync(settings).await?;
	Ok(())
}
