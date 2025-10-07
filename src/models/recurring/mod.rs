use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::trigger::Trade;




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


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CloseRecurringType {
  Time,
  Price,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelRecurringOrderReq {
  pub order: String,
  pub recurring_type: CloseRecurringType,
  pub user: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelRecurringOrderRes {
  pub request_id: String,
  pub transaction: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriceDepositReq {
  pub amount: String,
  pub order: String,
  pub user: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriceDepositRes {
  pub request_id: String,
  pub transaction: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub enum Withdrawal {
  In,
  Out,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriceWithdrawReq {
  pub amount: String,
  pub input_or_output: Withdrawal,
  pub order: String,
  pub user: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriceWithdrawRes {
  pub request_id: String,
  pub transaction: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub enum RecurringOrderType {
  Time,
  Price,
  All,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub enum OrderStatus {
  Active,
  History,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetRecurringOrdersReq {
  pub recurring_type: RecurringOrderType,
  pub order_status: OrderStatus,
  pub user: String,
  pub page: i64,
  pub include_failed_tx: bool,
  pub mint: Option<String>,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetRecurringOrdersRes {
  pub order_status: OrderStatus,
  pub page: i64,
  pub total_pages: i64,
  pub user: String,
  pub timer: Option<Vec<TimeOrder>>,
  pub price: Option<Vec<PriceOrder>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct TimeOrder {
  pub close_tx: String,
  pub created_at: String,
  pub cycle_frequency: String,
  pub in_amount_per_cycle: String,
  pub in_deposited: String,
  pub in_used: String,
  pub in_withdrawn: String,
  pub input_mint: String,
  pub max_out_amount: String,
  pub min_out_amount: String,
  pub open_tx: String,
  pub order_key: String,
  pub out_received: String,
  pub out_withdrawn: String,
  pub output_mint: String,
  pub raw_in_amount_per_cycl: String,
  pub raw_in_deposited: String,
  pub raw_in_used: String,
  pub raw_in_withdrawn: String,
  pub raw_max_out_amount: String,
  pub raw_min_out_amount: String,
  pub raw_out_received: String,
  pub raw_out_withdrawn: String,
  pub trades: Vec<Trade>,
  pub updated_at: String,
  pub user_closed: bool,
  pub user_pubkey: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriceOrder {
  pub close_tx: String,
  pub closed_by: String,
  pub created_by: String,
  pub estimated_usdc_value_spent: String,
  pub in_deposited: String,
  pub in_left: String,
  pub in_used: String,
  pub in_withdrawn: String,
  pub incremental_usd_value: String,
  pub input_mint: String,
  pub open_tx: String,
  pub order_interval: String,
  pub order_key: String,
  pub out_received: String,
  pub out_withdrawn: String,
  pub output_mint: String,
  pub raw_estimated_usdc_value_spent: String,
  pub raw_in_deposited: String,
  pub raw_in_left: String,
  pub raw_in_used: String,
  pub raw_in_withdrawn: String,
  pub raw_incremental_usd_value: String,
  pub raw_out_received: String,
  pub raw_out_withdrawn: String,
  pub raw_supposed_usd_value: String,
  pub start_at: String,
  pub status: String,
  pub supposed_usd_value: String,
  pub trades: Vec<Trade>,
  pub updated_at: String,
  pub user_pubkey: String,
}