use crate::CONFIG;
use log::{error, info, warn};
use matrix_sdk::{ruma::events::room::member::StrippedRoomMemberEvent, Client, Room};
use std::time::Duration;
use tokio::time::sleep;

/// auto join
pub async fn on_join(room_member: StrippedRoomMemberEvent, client: Client, room: Room) {
	if room_member.state_key != client.user_id().unwrap() {
		return;
	}

	if !CONFIG.matrix.user_allowed.is_allowed(&room_member.sender) {
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
