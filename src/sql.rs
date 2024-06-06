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

// return the database id of a matrix user.
// If the user not exist already it create a new one.
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

/// Make a user owner of a uploaded file
async fn add_owner_to_file(file_id: i64, user_id: i32) -> Result<(), sqlx::Error> {
	query!(
		"INSERT INTO file_owner(user_id, file_id) VALUES(($1), ($2))",
		user_id,
		file_id
	)
	.execute(&*SQL_POOL)
	.await?;
	Ok(())
}

/// To avoid duplicated uploads to matrix,
/// we track if a file with this hash was already uploaded.
/// If this the case we return the mxc of the file.
/// We add the user two the list of owner if he not already own the file
pub async fn get_mxc_file_by_hash_and_add_user_to_owner(
	hash: &mstickerlib::database::Hash,
	user_id: i32
) -> Result<Option<String>, sqlx::Error> {
	let entry = query!(
		r#"
		SELECT a.mxc, a.id, b.user_id as "user_id?"
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
			// file was already uploaded but is not owned by this user yet. Add user to owners
			add_owner_to_file(entry.id, user_id).await?;
		}
		return Ok(Some(entry.mxc));
	}
	Ok(None)
}

/// After sucesfull upload, the files hash and mcx url is added to database,
/// to avoid dulicated uploads.
/// Also the user make owner of the file
pub async fn add_mcx(
	hash: &mstickerlib::database::Hash,
	mxc: &str,
	user_id: i32
) -> Result<(), sqlx::Error> {
	let file_id = query!(
		"INSERT INTO files(hash, mxc) VALUES(($1), ($2)) RETURNING id",
		hash,
		mxc
	)
	.fetch_one(&*SQL_POOL)
	.await?;
	add_owner_to_file(file_id.id, user_id).await?;
	Ok(())
}
