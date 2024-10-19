use std::{env, path::PathBuf, process::Command};

fn run_command(cmd: &mut Command) -> String {
	let display = format!("{cmd:?}");
	let output = cmd.output().expect(&display);
	if !output.status.success() {
		let mut message = String::from_utf8_lossy(&output.stdout);
		if !message.is_empty() {
			message += "\n"
		}
		message += String::from_utf8_lossy(&output.stderr);
		panic!("Failed to execute {display}:\n{message}");
	}
	String::from_utf8_lossy(&output.stdout).into()
}

fn main() {
	//rerun on migrations change
	println!("cargo:rerun-if-changed=migrations");
	// set default for env DATABASE_URL
	let database_url = std::env::var("DATABASE_URL");
	let database_url = database_url.unwrap_or_else(|err| match err {
		env::VarError::NotPresent => "postgres://localhost/mstickerpicker".to_owned(),
		_ => panic!("Error reading environment variable DATABASE_URL")
	});
	println!("cargo:rustc-env=DATABASE_URL={database_url}");

	// clean database if it already exist
	let clean_up = false;
	if clean_up {
		let database_name = PathBuf::from(&database_url);
		let database_name = database_name.file_name().unwrap();
		let database_name = database_name.to_str().unwrap();
		for line in run_command(Command::new("psql").arg("-lqtA")).lines() {
			if line.starts_with(&format!("{database_name}|")) {
				run_command(Command::new("dropdb").arg(database_name));
				break;
			}
		}
		run_command(Command::new("createdb").arg(database_name));
	}

	// run migrations using sqlx
	run_command(Command::new("sqlx").args([
		"migrate",
		"run",
		"--database-url",
		&database_url,
		"--source",
		"src/migrations/"
	]));
}
