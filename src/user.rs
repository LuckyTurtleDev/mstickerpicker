use crate::CONFIG;
use rocket::{self, http::Status};

#[post("/register", data = "<token>")]
pub(crate) async fn register(token: String) -> Result<String, Status> {
	if token == CONFIG.register_token {
		return Ok("TODO".to_owned());
	}
	Err(Status::Unauthorized)
}
