use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub evm: EvmConfig,
    pub cosmos: CosmosConfig,
}

#[derive(Debug, Deserialize)]
pub struct EvmConfig {
    pub rpc_url: String,
    pub chain_id: u64,
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct CosmosConfig {
    pub grpc_url: String,
    pub chain_id: String,
    pub private_key: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_file = std::fs::File::open("config.json")?;
        let mut contents = String::new();
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}