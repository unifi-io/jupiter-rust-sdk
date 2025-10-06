use serde::{Deserialize, Serialize};

use crate::{tx::ExecuteRes, JupiterClient, JupiterError};




#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub signed_transaction: String, // The signed transaction to execute
    pub request_id: String, // Found in response of /order
}





#[derive(Clone)]
pub struct UltraClient(JupiterClient);

// #[async_trait]
impl UltraClient {
    pub fn new(client: JupiterClient) -> Self {
        Self(client)
    }

    pub async fn execute(
        &self,
        req: &TransactionRequest,
    ) -> Result<ExecuteRes, JupiterError> {
        let path = "/ultra/v1/execute";
        self.0.post(&path, req).await
    }
}




#[cfg(test)]
mod tests {
    use crate::OrderReq;

    use super::*;

    #[tokio::test]
    async fn test_execute() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let res = client.get_order(&OrderReq {
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

        let ultra_client = UltraClient::new(client);

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
}