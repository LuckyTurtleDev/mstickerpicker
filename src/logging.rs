use colored::*;
use once_cell::sync::Lazy;
use rocket::{
	config::LogLevel,
	tokio::task::{spawn_blocking, JoinHandle},
};

pub static LOG_LEVEL: Lazy<LogLevel> = Lazy::new(|| {
	rocket::Config::figment()
		.extract_inner("log_level")
		.expect("failed to get log level")
});

pub fn force_init() -> JoinHandle<()> {
	spawn_blocking(|| {
		Lazy::force(&LOG_LEVEL);
	})
}

pub fn log_error<T>(message: T)
where
	T: std::fmt::Display,
{
	if *LOG_LEVEL != LogLevel::Off {
		eprintln!("   {} {}", ">>".bold(), message.to_string().red());
	}
}
