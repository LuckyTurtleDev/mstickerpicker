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

pub async fn get_file_mxc(
	hash: &mstickerlib::database::Hash,
	user_id: i32
) -> Result<Option<String>, sqlx::Error> {
	let entry = query!(
		r#"
		SELECT a.mxc, a.id, b.user_id
		FROM files AS a LEFT JOIN file_owner AS b
		ON a.id = b.file_id
		WHERE a.hash=($1) AND (b.user_id = ($2) OR b.user_id IS NULL)
		"#,
		hash,
		user_id
	)
	.fetch_optional(&*SQL_POOL)
	.await?;
	if let Some(entry) = entry {
		if entry.user_id.is_none() {
			// file was already uploaded but is not owned by this user yet. Add user to oweners
			query!(
				"INSERT INTO file_owner(user_id, file_id) VALUES(($1), ($2))",
				user_id,
				entry.id
			)
			.execute(&*SQL_POOL)
			.await?;
		}
		return Ok(Some(entry.mxc));
	}
	Ok(None)
}
