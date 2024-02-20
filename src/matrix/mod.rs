use crate::load_env;
use log::{error, info, warn};
use matrix_sdk::{
	config::SyncSettings,
	ruma::{
		events::room::{
			member::StrippedRoomMemberEvent,
			message::{
				MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent
			}
		},
		OwnedServerName, OwnedUserId, ServerName, UserId
	},
	Client, Room, RoomState
};
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::time::sleep;

async fn on_room_message(
	event: OriginalSyncRoomMessageEvent,
	room: Room,
	client: Client,
	config: Arc<MatrixConfig>
) {
	if room.state() != RoomState::Joined {
		return;
	}
	let MessageType::Text(text_content) = event.content.msgtype else {
		return;
	};
	if event.sender == client.user_id().unwrap() {
		return;
	}

	if !config.user_allowed.is_allowed(&event.sender) {
		let content = RoomMessageEventContent::text_plain(
			"Error: You have no permission to use this bot"
		);
		room.send(content).await.unwrap();
		return;
	}

	if text_content.body.contains("!party") {
		let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");
		info!("sending");
		// send our message to the room we found the "!party" command in
		room.send(content).await.unwrap();
		info!("message sent");
	}
}

/// auto join
async fn on_join(
	room_member: StrippedRoomMemberEvent,
	client: Client,
	room: Room,
	config: Arc<MatrixConfig>
) {
	if room_member.state_key != client.user_id().unwrap() {
		return;
	}

	if !config.user_allowed.is_allowed(&room_member.sender) {
		return;
	}

	tokio::spawn(async move {
		info!("Autojoining room {}", room.room_id());
		let mut delay = 2;

		while let Err(err) = room.join().await {
			// retry autojoin due to synapse sending invites, before the
			// invited user can join for more information see
			// https://github.com/matrix-org/synapse/issues/4345
			warn!(
				"Failed to join room {} ({err:?}), retrying in {delay}s",
				room.room_id()
			);

			sleep(Duration::from_secs(delay)).await;
			delay *= 2;

			if delay > 3600 {
				error!("Can't join room {} ({err:?})", room.room_id());
				break;
			}
		}
		info!("Successfully joined room {}", room.room_id());
	});
}

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

pub async fn start_matrix(config: MatrixConfig) -> anyhow::Result<()> {
	info!("{config:#?}");
	let config = Arc::new(config);
	// Note that when encryption is enabled, you should use a persistent store to be
	// able to restore the session with a working encryption setup.
	// See the `persist_session` example.
	let client = Client::builder()
		.homeserver_url(&config.homeserver_url)
		.build()
		.await?;
	client
		.matrix_auth()
		.login_username(&config.username, &config.password)
		.initial_device_display_name("command bot")
		.await?;

	info!("logged in at matrix as {}", config.username);

	// An initial sync to set up state and so our bot doesn't respond to old
	// messages. But we want to still process invites.
	let config_tmp = config.clone();
	client.add_event_handler(
		|room_member: StrippedRoomMemberEvent, client: Client, room: Room| async move {
			on_join(room_member, client, room, config_tmp).await
		}
	);
	let response = client.sync_once(SyncSettings::default()).await?;
	// add our CommandBot to be notified of incoming messages, we do this after the
	// initial sync to avoid responding to messages before the bot was running.
	client.add_event_handler(
		|event: OriginalSyncRoomMessageEvent, client: Client, room: Room| async move {
			on_room_message(event, room, client, config).await
		}
	);

	// since we called `sync_once` before we entered our sync loop we must pass
	// that sync token to `sync`
	let settings = SyncSettings::default().token(response.next_batch);
	// this keeps state from the server streaming in to CommandBot via the
	// EventHandler trait
	client.sync(settings).await?;

	Ok(())
}
