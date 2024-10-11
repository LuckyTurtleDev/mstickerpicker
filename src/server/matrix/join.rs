use anyhow::Context;
use log::{error, info, warn};
use matrix_sdk::{
	ruma::events::room::{
		member::StrippedRoomMemberEvent, message::RoomMessageEventContent
	},
	Client, Room
};
use once_cell::sync::Lazy;

use std::time::Duration;
use tokio::time::sleep;

//use super::USER_ALLOWED;

/// auto join
pub async fn on_join(room_member: StrippedRoomMemberEvent, client: Client, room: Room) {
	if room_member.state_key != client.user_id().unwrap() {
		return;
	}

	//if !CONFIG
	//	.matrix
	//	.user_allowed
	//	.is_allowed_ignore_err(&room_member.sender)
	//	.await
	//{
	//	return;
	//}

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
				let err = anyhow::Error::from(err)
					.context(format!("Can't join room {}", room.room_id()));
				error!("{err:?}");
				break;
			}
		}
		info!("Successfully joined room {}", room.room_id());

		// welcome message
		static JOIN_MESSAGE: Lazy<RoomMessageEventContent> =
			Lazy::new(|| RoomMessageEventContent::text_plain("todo"));
		if let Err(err) = room
			.send((*JOIN_MESSAGE).to_owned())
			.await
			.context("failed to join room")
		{
			error!("{err:?}");
		}
	});

}