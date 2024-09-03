use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use thiserror::Error;

use crate::config::EvmConfig;

#[derive(Error, Debug)]
pub enum EvmClientError {
    #[error("Failed to connect to EVM node: {0}")]
    ConnectionError(#[from] ProviderError),
    #[error("Transaction error: {0}")]
    TransactionError(#[from] ContractError<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>),
}

pub struct EvmClient {
    client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
}

impl EvmClient {
    pub async fn new(config: &EvmConfig) -> Result<Self, EvmClientError> {
        let provider = Provider::<Http>::try_from(&config.rpc_url).unwrap();
        let chain_id = config.chain_id;
        let wallet: LocalWallet = config.private_key.parse().unwrap();
        let wallet = wallet.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider, wallet);

        Ok(Self {
            client: client.into(),
        })
    }

    pub async fn get_latest_block(&self) -> Result<Block<TxHash>, EvmClientError> {
        let block = self.client.get_block(BlockNumber::Latest).await.unwrap();
        Ok(block.unwrap())
    }

}
