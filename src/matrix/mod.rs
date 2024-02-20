use log::{error, info, warn};
use matrix_sdk::{
	config::SyncSettings,
	ruma::events::room::{
		member::StrippedRoomMemberEvent,
		message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent}
	},
	Client, Room, RoomState
};
use std::time::Duration;
use tokio::time::sleep;

async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
	if room.state() != RoomState::Joined {
		return;
	}
	let MessageType::Text(text_content) = event.content.msgtype else {
		return;
	};

	if text_content.body.contains("!party") {
		let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");
		info!("sending");
		// send our message to the room we found the "!party" command in
		room.send(content).await.unwrap();
		info!("message sent");
	}
}

/// auto join
async fn on_stripped_state_member(
	room_member: StrippedRoomMemberEvent,
	client: Client,
	room: Room
) {
	if room_member.state_key != client.user_id().unwrap() {
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

pub struct MatrixConfig {
	pub homeserver_url: String,
	pub username: String,
	pub password: String
}

pub async fn start_matrix(config: MatrixConfig) -> anyhow::Result<()> {
	// Note that when encryption is enabled, you should use a persistent store to be
	// able to restore the session with a working encryption setup.
	// See the `persist_session` example.
	let client = Client::builder()
		.homeserver_url(config.homeserver_url)
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
	client.add_event_handler(on_stripped_state_member);
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
