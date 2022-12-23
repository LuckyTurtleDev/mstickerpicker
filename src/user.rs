use crate::{CONFIG, SQL_POOL};
use rand::RngCore;
use rocket::{self, http::Status};

#[post("/register", data = "<reg_token>")]
pub(crate) async fn register(reg_token: Vec<u8>) -> Result<Vec<u8>, Status> {
	//TODO: error handling
	if reg_token == CONFIG.register_token.as_bytes() {
		let mut user_token = [0u8; 128];
		rand::thread_rng().fill_bytes(&mut user_token);
		sqlx::query!(
			"INSERT INTO users (id)
			VALUES ($1);",
			&user_token
		)
		.execute(&*SQL_POOL)
		.await
		.unwrap(); //TODO: error handling
		return Ok(user_token.to_vec());
	}
	Err(Status::Unauthorized)
}
