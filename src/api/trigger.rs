use crate::{
    api::TransactionRequest, 
    trigger::{
        CancelOrderReq, 
        CancelOrderRes, 
        CancelOrdersReq, 
        CancelOrdersRes, 
        CreateOrderReq, 
        CreateOrderRes, 
        GetTriggerOrdersReq, 
        GetTriggerOrdersRes,
    }, 
    tx::ExecuteRes, 
    JupiterClient, 
    JupiterError,
};





#[derive(Clone)]
pub struct TriggerService<'a>{
    client: &'a JupiterClient,
}


// #[async_trait]
impl<'a> TriggerService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }


    pub async fn create_order(
        &self,
        req: &CreateOrderReq,
    ) -> Result<CreateOrderRes, JupiterError> {
        let path = "/lend/v1/earn/positions";
        self.client.get_json_with_query(&path, req).await
    }

    pub async fn execute(
        &self,
        req: &TransactionRequest,
    ) -> Result<ExecuteRes, JupiterError> {
        let path = "/trigger/v1/execute";
        self.client.post(&path, req).await
    }

    pub async fn cancel_order(
        &self,
        req: &CancelOrderReq,
    ) -> Result<CancelOrderRes, JupiterError> {
        let path = "/trigger/v1/cancelOrder";
        self.client.post(&path, req).await
    }

    pub async fn cancel_orders(
        &self,
        req: &CancelOrdersReq,
    ) -> Result<CancelOrdersRes, JupiterError> {
        let path = "/trigger/v1/cancelOrders";
        self.client.post(&path, req).await
    }

    pub async fn get_trigger_orders(
        &self,
        req: &GetTriggerOrdersReq,
    ) -> Result<GetTriggerOrdersRes, JupiterError> {
        let path = "/trigger/v1/getTriggerOrders";
        self.client.get_json_with_query(&path, req).await
    }
}




#[cfg(test)]
mod tests {

    use crate::trigger::CreateOrderParams;

    use super::*;

    #[tokio::test]
    async fn test_create_order() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let trigger_service = TriggerService::new(&client);

        let res = trigger_service.create_order(&CreateOrderReq {
            input_mint: "".to_string(),
            output_mint: "".to_string(),
            maker: "".to_string(),
            payer: "".to_string(),
            params: CreateOrderParams{
                making_amount: "".to_string(),
                taking_amount: "".to_string(),
                expired_at: "".to_string(),
                slippage_bps: "".to_string(),
                fee_bps: "".to_string(),
            },
            compute_unit_price: None,
            fee_account: None,
            wrap_and_unwrap_sol: None,
        }).await;

        match res {
            Ok(res) => {
                // assert_eq!(res.error_code, None);
                println!("create_order: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("create_order error: {}", e);
            }
        }
    }
}

