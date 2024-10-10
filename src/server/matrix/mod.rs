use std::time::Duration;
use tokio::time::sleep;

pub async fn start_matrix() -> anyhow::Result<()> {
    loop {
        sleep(Duration::from_secs(3)).await;
        log::info!("halloooooooo?");
    }
	Ok(())
}
