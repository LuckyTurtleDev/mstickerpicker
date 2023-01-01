use crate::{error::ServerError, CONFIG, SQL_POOL};
use actix_web::post;
use rand::RngCore;

#[post("/register")]
pub(crate) async fn register(reg_token: actix_web::web::Bytes) -> Result<Vec<u8>, ServerError> {
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
	Err(ServerError::WrongToken)
}
