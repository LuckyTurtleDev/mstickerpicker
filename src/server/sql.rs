use anyhow::Context;
use matrix_sdk::ruma::UserId;
use once_cell::sync::OnceCell;
use rand::{distributions, thread_rng, Rng};
use sqlx::{query, Pool, Postgres};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

fn pool() -> &'static Pool<Postgres> {
	POOL.get().unwrap()
}

/// must be called exactly once before the first usage of the datbase
pub async fn init_pool() -> anyhow::Result<()> {
	let pool = sqlx::PgPool::connect("postgres://localhost/mstickerpicker")
		.await
		.context("can not connect to database")?;
	sqlx::migrate!("src/migrations")
		.run(&pool)
		.await
		.context("database migration has failed")?;
	POOL.set(pool).unwrap();
	Ok(())
}

/// return the database id of a matrix user if user already exist at the database
pub async fn try_get_user_id(user: &UserId) -> Result<Option<i32>, sqlx::Error> {
	query!("SELECT id FROM users WHERE mxid=($1)", format!("{user}"))
		.fetch_optional(POOL.get().unwrap())
		.await
		.map(|r| r.map(|o| o.id))
}

/// return the database id of a matrix user.
/// If the user not exist already it create a new one.
pub async fn get_or_creat_user_id(user: &UserId) -> Result<i32, sqlx::Error> {
	let pool = POOL.get().unwrap();
	let user_str = format!("{user}");
	let id = query!("SELECT id FROM users WHERE mxid=($1)", user_str)
		.fetch_optional(pool)
		.await?
		.map(|f| f.id);
	let id = match id {
		Some(id) => id,
		None => {
			query!("INSERT INTO users(mxid) VALUES($1) RETURNING id", user_str)
				.fetch_one(pool)
				.await?
				.id
		},
	};
	Ok(id)
}

/// create and add an new web token to the database.
/// Return the created token
pub async fn add_new_token(user_id: i32) -> Result<String, sqlx::Error> {
	let token: String = thread_rng()
		.sample_iter(&distributions::Alphanumeric)
		.take(64)
		.map(char::from)
		.collect();
	query!(
		"INSERT INTO token(token, user_id) Values($1, $2)",
		token,
		user_id
	)
	.execute(pool())
	.await?;
	Ok(token)
}
