use sni_tray::service::{StatusNotifierHost, StatusNotifierWatcher};
use std::error::Error;
use tokio::{signal, sync::mpsc};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    console_subscriber::init();
    let token = CancellationToken::new();
    let ctrl_c = {
        let token = token.clone();
        tokio::spawn(async move {
            signal::ctrl_c().await.unwrap();
            token.cancel();
        })
    };
    let (tx, mut rx) = mpsc::channel(32);
    let watcher = tokio::spawn(StatusNotifierWatcher::new().run(token.child_token()));
    let host = tokio::spawn(StatusNotifierHost::new().run(token.child_token(), tx));
    while let Some(message) = rx.recv().await {
        println!("{:?}", message);
    }
    let _ = watcher.await;
    let _ = host.await;
    let _ = ctrl_c.await;
    Ok(())
}
