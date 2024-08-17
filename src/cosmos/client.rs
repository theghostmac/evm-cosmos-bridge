use crate::config::CosmosConfig;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetLatestBlockRequest;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient;
use tonic::transport::Channel;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CosmosClientError {
    #[error("Failed to connect to Cosmos node: {0}")]
    ConnectionError(#[from] tonic::transport::Error),
    #[error("Failed to get latest block: {0}")]
    BlockQueryError(#[from] tonic::Status),
}

pub struct CosmosClient {
    client: ServiceClient<Channel>,
}

impl CosmosClient {
    pub async fn new(config: &CosmosConfig) -> Result<Self, CosmosClientError> {
        let channel = Channel::from_shared(config.grpc_url.clone())?.connect().await?;
        let client = ServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn get_latest_block(&mut self) -> Result<cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::Block, CosmosClientError> {
        let request = tonic::Request::new(GetLatestBlockRequest {});
        let response = self.client.get_latest_block(request).await?;
        Ok(response.into_inner().block.unwrap())
    }

    // Add more methods for interacting with Cosmos chain and modules.
}