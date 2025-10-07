use crate::{
    api::TransactionRequest, common::ExecuteRes, recurring::{
        CancelRecurringOrderReq, 
        CancelRecurringOrderRes, 
        CreateRecurringOrderReq, 
        CreateRecurringOrderRes, 
        GetRecurringOrdersReq, 
        GetRecurringOrdersRes, 
        PriceDepositReq, 
        PriceDepositRes, 
        PriceWithdrawReq, 
        PriceWithdrawRes,
    }, JupiterClient, JupiterError
};




#[derive(Clone)]
pub struct RecurringService<'a>{
    client: &'a JupiterClient,
}


impl<'a> RecurringService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }


    pub async fn create_order(
        &self,
        req: &CreateRecurringOrderReq,
    ) -> Result<CreateRecurringOrderRes, JupiterError> {
        let path = "/recurring/v1/createOrder";
        self.client.post(&path, req).await
    }


    pub async fn execute(
        &self,
        req: &TransactionRequest,
    ) -> Result<ExecuteRes, JupiterError> {
        let path = "/recurring/v1/execute";
        self.client.post(&path, req).await
    }

    pub async fn cancel_order(
        &self,
        req: &CancelRecurringOrderReq,
    ) -> Result<CancelRecurringOrderRes, JupiterError> {
        let path = "/recurring/v1/cancelOrder";
        self.client.post(&path, req).await
    }

    pub async fn price_deposit(
        &self,
        req: &PriceDepositReq,
    ) -> Result<PriceDepositRes, JupiterError> {
        let path = "/recurring/v1/priceDeposit";
        self.client.post(&path, req).await
    }

    pub async fn price_withdraw(
        &self,
        req: &PriceWithdrawReq,
    ) -> Result<PriceWithdrawRes, JupiterError> {
        let path = "/recurring/v1/priceWithdraw";
        self.client.post(&path, req).await
    }

    pub async fn get_recurring_orders(
        &self,
        req: &GetRecurringOrdersReq,
    ) -> Result<GetRecurringOrdersRes, JupiterError> {
        let path = "/recurring/v1/getRecurringOrders";
        self.client.get_json_with_query(&path, req).await
    }
}