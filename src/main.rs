use sni_tray::notifier_watcher::StatusNotifierWatcher;
use std::error::Error;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().pretty().init();
    StatusNotifierWatcher::run(CancellationToken::new()).await?;
    Ok(())
}
