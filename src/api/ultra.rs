use serde::{Deserialize, Serialize};

use crate::{common::ExecuteRes, models::ultra::GetHoldingsRes, ultra::order::{OrderReq, OrderRes}, JupiterClient, JupiterError};




#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub signed_transaction: String, // The signed transaction to execute
    pub request_id: String, // Found in response of /order
}





#[derive(Clone)]
pub struct UltraService<'a>{
    client: &'a JupiterClient,
}

// #[async_trait]
impl<'a> UltraService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }

    pub async fn get_order(&self, req: &OrderReq) -> Result<OrderRes, JupiterError> {
        let path = "/ultra/v1/order";
        self.client.get_json_with_query(&path, req,).await
    }

    pub async fn execute(
        &self,
        req: &TransactionRequest,
    ) -> Result<ExecuteRes, JupiterError> {
        let path = "/ultra/v1/execute";
        self.client.post(&path, req).await
    }


    /**
     * Request for detailed token holdings of an account including token account information
     */
    pub async fn get_holdings(
        &self,
        address: String,
    ) -> Result<GetHoldingsRes, JupiterError> {
        let path = format!("/ultra/v1/holdings/{}", address);
        self.client.get_json(&path).await
    }
}




#[cfg(test)]
mod tests {
    use crate::ultra::order::OrderReq;

    use super::*;

    #[tokio::test]
    async fn test_execute() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let ultra_service = UltraService::new(&client);
        let res = ultra_service.get_order(&OrderReq {
            input_mint: "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".to_string(),
            output_mint: "So11111111111111111111111111111111111111112".to_string(),
            amount: "500".to_string(),
            taker: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
            referral_account: None,
            referral_fee: None,
            exclude_dexes: None,
            exclude_routers: None,
        }).await.unwrap();

        assert_eq!(res.error_code, None);
        println!("order: {}", serde_json::to_string_pretty(&res).unwrap());

        let ultra_client = UltraService::new(&client);

        let res = ultra_client.execute(&TransactionRequest {
            signed_transaction: res.transaction.unwrap(),
            request_id: res.request_id,
        }).await;

        match res {
            Ok(res) => {
                assert_eq!(res.error_code, None);
                println!("execute: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("execute error: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_holdings() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let ultra_client = UltraService::new(&client);
        let res = ultra_client.get_holdings("ANAUcDCU3Jfao3mtxBdttjEH7F3Ja7SyjGBKUa9Cruc5".to_string()).await.unwrap();
        assert_eq!(res.error_code, None);
        println!("holdings: {}", serde_json::to_string_pretty(&res).unwrap());
    }

    #[tokio::test]
    async fn test_get_order() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let ultra_service = UltraService::new(&client);
        let res = ultra_service.get_order(&OrderReq {
            input_mint: "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".to_string(),
            output_mint: "So11111111111111111111111111111111111111112".to_string(),
            amount: "500".to_string(),
            taker: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
            referral_account: None,
            referral_fee: None,
            exclude_dexes: None,
            exclude_routers: None,
        }).await.unwrap();

        assert_eq!(res.error_code, None);
        println!("order: {}", serde_json::to_string_pretty(&res).unwrap());
    }
}