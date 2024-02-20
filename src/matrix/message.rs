use crate::MatrixConfig;
use log::info;
use matrix_sdk::{
	ruma::events::room::message::{
		AddMentions, ForwardThread, MessageType, OriginalSyncRoomMessageEvent,
		RoomMessageEventContent
	},
	Client, Room, RoomState
};
use std::sync::Arc;

pub async fn on_room_message(
	event: OriginalSyncRoomMessageEvent,
	room: Room,
	client: Client,
	config: Arc<MatrixConfig>
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
	if !config.user_allowed.is_allowed(&event.sender) {
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

	if text_content.body.contains("!party") {
		let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉")
			.make_reply_to(
				&event.into_full_event(room.room_id().into()),
				ForwardThread::No,
				AddMentions::Yes
			);
		info!("sending");
		// send our message to the room we found the "!party" command in
		room.send(content).await.unwrap();
		info!("message sent");
	}
}
