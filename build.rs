use std::{env, process::Command};

fn main() {
	//rerun on migrations change
	println!("cargo:rerun-if-changed=migrations");
	// set default for env DATABASE_URL
	let database_url = std::env::var("DATABASE_URL");
	let database_url = database_url.unwrap_or_else(|err| match err {
		env::VarError::NotPresent => "postgres://localhost/mstickerpicker".to_owned(),
		_ => Err(err).expect("Error reading environment varible DATABASE_URL"),
	});
	println!("cargo:rustc-env=DATABASE_URL={database_url}");
	// run migrations using sqlx
	let output = Command::new("sqlx")
		.args([
			"migrate",
			"run",
			"--database-url",
			&database_url,
			"--source",
			"src/migrations/",
		])
		.output()
		.expect("failed to run sqlx");
	if !output.status.success() {
		let mut message = String::from_utf8_lossy(&output.stdout);
		if !message.is_empty() {
			message += "\n"
		}
		message += String::from_utf8_lossy(&output.stderr);
		panic!("Failed to execute sqlx:\n{message}");
	}
}
