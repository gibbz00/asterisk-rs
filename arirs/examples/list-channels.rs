use arirs::{RequestClient, Result};
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = RequestClient::default();

    for channel in client.list().await? {
        debug!("Channel ID: {}", channel.id);
    }

    Ok(())
}
