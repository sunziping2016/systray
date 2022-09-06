use futures::StreamExt;
use systray::service::{NotifierHost, NotifierWatcher};
use std::error::Error;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let token = CancellationToken::new();
    let watcher = NotifierWatcher::new()
        .cancellable(token.child_token())
        .run()
        .await?;
    let host = NotifierHost::new()
        .cancellable(token.child_token())
        .run()
        .await?;
    tokio::join! {
        watcher,
        async move {
            tokio::pin!(host);
            while let Some(msg) = host.next().await {
                println!("{:?}", msg);
            }
        },
        async move {
            signal::ctrl_c().await.unwrap();
            token.cancel();
        },
    };
    Ok(())
}
