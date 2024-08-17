mod config;
mod evm;
mod cosmos;
mod bridge;

use std::error::Error;
use crate::bridge::relay::BridgeRelay;
use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initializing the logging.
    tracing_subscriber::fmt::init();

    // Loading config.
    let config = Config::load()?;

    // Initializing the bridge relay.
    let bridge_relay = BridgeRelay::new(config)?;

    // Start the bridge relay.
    bridge_relay.run().await?;

    Ok(())
}