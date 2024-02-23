use crate::SQL_POOL;
use matrix_sdk::ruma::UserId;
use sqlx::query;

/// if sucess return option from user id
pub async fn try_get_user_id(user: &UserId) -> Result<Option<i32>, sqlx::Error> {
	query!("SELECT id FROM users WHERE mxid=($1)", format!("{user}"))
		.fetch_optional(&*SQL_POOL)
		.await
		.map(|r| r.map(|o| o.id))
}

pub async fn get_or_creat_user_id(user: &UserId) -> Result<i32, sqlx::Error> {
	let user_str = format!("{user}");
	let id = query!("SELECT id FROM users WHERE mxid=($1)", user_str)
		.fetch_optional(&*SQL_POOL)
		.await?
		.map(|f| f.id);
	let id = match id {
		Some(id) => id,
		None => {
			query!("INSERT INTO users(mxid) VALUES($1) RETURNING id", user_str)
				.fetch_one(&*SQL_POOL)
				.await?
				.id
		},
	};
	Ok(id)
}
