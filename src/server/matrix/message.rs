use log::error;
use matrix_sdk::{
	ruma::events::room::message::{
		AddMentions, ForwardThread, MessageType, OriginalSyncRoomMessageEvent,
		RoomMessageEventContent
	},
	Client, Room, RoomState
};

use super::USER_ALLOWED;
use crate::server::{error::MyContext, sql};

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

	let mut outer_user_id = None;
	if !USER_ALLOWED.get().unwrap().allowed(&event.sender) {
		//check also it the user already exist at the database / is register already
		let user_id = match sql::try_get_user_id(&event.sender)
			.await
			.my_context("failed to look up user at the database")
		{
			Ok(value) => value,
			Err(err) => {
				err.log();
				make_reply_and_send(err.matrix_event(), event, &room).await;
				return;
			}
		};
		match user_id {
			None => {
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
			Some(user_id) => outer_user_id = Some(user_id)
		}
	}

	// execute the cli
	let message = super::cli::execute_cli(
		client.user_id().unwrap(),
		&event.sender,
		outer_user_id,
		&text_content.body
	)
	.await;
	make_reply_and_send(message, event, &room).await;
}
