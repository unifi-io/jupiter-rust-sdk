use crate::{studio::{DbcFeeCreateTxReq, DbcFeeCreateTxRes, GetDbcFeeReq, GetDbcFeeRes, GetPoolByMintRes, PoolCreateTxReq, PoolSubmitReq, PoolSubmitRes}, JupiterClient, JupiterError};





#[derive(Clone)]
pub struct StudioService<'a>{
    client: &'a JupiterClient,
}


impl<'a> StudioService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }

    pub async fn pool_create_tx(
        &self,
        req: &PoolCreateTxReq,
    ) -> Result<PoolCreateTxReq, JupiterError> {
        let path = "/studio/v1/dbc-pool/create-tx";
        self.client.post(&path, req).await
    }

    pub async fn pool_submit(
        &self,
        req: &PoolSubmitReq,
    ) -> Result<PoolSubmitRes, JupiterError> {
        let path = "/studio/v1/dbc-pool/submit";
        self.client.post(&path, req).await
    }

    pub async fn get_pool_by_mint(
        &self,
        mint: String,
    ) -> Result<GetPoolByMintRes, JupiterError> {
        let path = format!("/studio/v1/dbc-pool/addresses/{}", mint);
        self.client.get_json(&path).await
    }

    pub async fn get_dbc_fee(
        &self,
        req: &GetDbcFeeReq,
    ) -> Result<GetDbcFeeRes, JupiterError> {
        let path = "/studio/v1/dbc/fee";
        self.client.get_json_with_query(&path, req).await
    }

    pub async fn dbc_fee_create_tx(
        &self,
        req: &DbcFeeCreateTxReq,
    ) -> Result<DbcFeeCreateTxRes, JupiterError> {
        let path = "/studio/v1/dbc/fee/create-tx";
        self.client.post(&path, req).await
    }
}