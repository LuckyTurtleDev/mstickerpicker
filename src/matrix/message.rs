use super::cli::execute_cli;
use crate::CONFIG;
use log::info;
use matrix_sdk::{
	ruma::events::room::message::{
		AddMentions, ForwardThread, MessageType, OriginalSyncRoomMessageEvent,
		RoomMessageEventContent
	},
	Client, Room, RoomState
};

pub async fn on_room_message(
	event: OriginalSyncRoomMessageEvent,
	room: Room,
	client: Client
) {
	if room.state() != RoomState::Joined {
		return;
	}
	let MessageType::Text(ref text_content) = event.content.msgtype else {
		return;
	};
	if event.sender == client.user_id().unwrap() {
		return;
	}
	if !CONFIG.matrix.user_allowed.is_allowed(&event.sender) {
		let content = RoomMessageEventContent::text_plain(
			"Error: You have no permission to use this bot"
		)
		.make_reply_to(
			&event.into_full_event(room.room_id().into()),
			ForwardThread::No,
			AddMentions::Yes
		);
		room.send(content).await.unwrap();
		return;
	}

	let content = match execute_cli(client.user_id().unwrap(), &text_content.body) {
		Ok(value) => value,
		Err(err) => RoomMessageEventContent::text_plain(format!("{err:?}"))
	};
	let content = content.make_reply_to(
		&event.into_full_event(room.room_id().into()),
		ForwardThread::No,
		AddMentions::Yes
	);
	info!("sending");
	room.send(content).await.unwrap();
	info!("message sent");
}
