use crate::{error::Error, CONFIG};
use log::error;
use matrix_sdk::{
	ruma::events::room::message::{
		AddMentions, ForwardThread, MessageType, OriginalSyncRoomMessageEvent,
		RoomMessageEventContent
	},
	Client, Room, RoomState
};

use super::cli::execute_cli;

async fn make_reply_and_send(
	content: RoomMessageEventContent,
	original_message: OriginalSyncRoomMessageEvent,
	room: &Room
) {
	let content = content.make_reply_to(
		&original_message.into_full_event(room.room_id().into()),
		ForwardThread::No,
		AddMentions::Yes
	);
	if let Err(err) = room.send(content).await {
		error!("{err}");
	}
}

pub async fn on_room_message(
	event: OriginalSyncRoomMessageEvent,
	room: Room,
	client: Client
) {
	// filter input message:
	// * the bot should be inside the room
	// * the message should not be send by the bot themself
	// * we want to ignore edits (and replies)
	// * the user must be allowed to use this bot
	if room.state() != RoomState::Joined {
		return;
	}
	if event.sender == client.user_id().unwrap() {
		return;
	}
	if event.content.relates_to.is_some() {
		return;
	}
	let MessageType::Text(ref text_content) = event.content.msgtype else {
		return;
	};
	let allowed = CONFIG
		.matrix
		.user_allowed
		.is_allowed(&event.sender)
		.await
		.map_err(|err| Error::from(err));
	match allowed {
		Ok(true) => {},
		Ok(false) => {
			make_reply_and_send(
				RoomMessageEventContent::text_plain(
					"Error: You have no permission to use this bot"
				),
				event,
				&room
			)
			.await;
			return;
		},
		Err(err) => {
			make_reply_and_send(err.into(), event, &room).await;
			return;
		}
	}

	let content =
		execute_cli(client.user_id().unwrap(), &text_content.body, &event.sender).await;
	make_reply_and_send(content, event, &room).await;
}
