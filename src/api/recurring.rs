use crate::{recurring::{CreateRecurringOrderReq, CreateRecurringOrderRes}, JupiterClient, JupiterError};




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
        self.client.get_json_with_query(&path, req).await
    }
}