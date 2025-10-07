use serde::{Deserialize, Serialize};
use serde_with::serde_as;




#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateRecurringOrderReq {
    pub input_mint: String,
    pub output_mint: String,
    pub params: CreateRecurringOrderParams,
    pub user: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateRecurringOrderParams {
  pub in_amount: i64,
  pub interval: i64,
  pub max_price: Option<f64>,
  pub min_price: Option<f64>,
  pub number_of_orders: i64, 
  pub start_at: i64,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateRecurringOrderRes {
    pub request_id: String,
    pub transaction: String,
}
