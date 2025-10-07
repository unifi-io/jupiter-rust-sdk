use std::collections::HashMap;

use crate::{swap::{SwapInstructionsRes, SwapQuoteReq, SwapQuoteRes, SwapReq, SwapRes}, JupiterClient, JupiterError};









#[derive(Clone)]
pub struct SwapService<'a>{
    client: &'a JupiterClient,
}


impl<'a> SwapService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }

    pub async fn quote(
        &self,
        req: &SwapQuoteReq,
    ) -> Result<SwapQuoteRes, JupiterError> {
        let path = "/swap/v1/quote";
        self.client.post(&path, req).await
    }

    pub async fn swap(
        &self,
        req: &SwapReq,
    ) -> Result<SwapRes, JupiterError> {
        let path = "/swap/v1/swap";
        self.client.post(&path, req).await
    }

    pub async fn swap_instructions(
        &self,
        req: &SwapReq,
    ) -> Result<SwapInstructionsRes, JupiterError> {
        let path = "/swap/v1/swap-instructions";
        self.client.post(&path, req).await
    }

    pub async fn program_id_to_label(
        &self,
    ) -> Result<HashMap<String, String>, JupiterError> {
        let path = "/swap/v1/program-id-to-label";
        self.client.get_json(&path).await        
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_program_id_to_label() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let swap_service = SwapService::new(&client);

        let res = swap_service.program_id_to_label().await;
        match res {
            Ok(res) => {
                println!("program_id_to_label: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("program_id_to_label error: {}", e);
            }
        }
    }
}

