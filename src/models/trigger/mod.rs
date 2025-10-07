use serde::{Deserialize, Serialize};
use serde_with::serde_as;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateOrderParams {
    pub making_amount: String,
    pub taking_amount: String,
    pub expired_at: String,
    // Amount of slippage the order can be executed with
    pub slippage_bps: String,
    // Requires the feeAccount parameter, the amount of fees in bps that will be sent to the fee account
    pub fee_bps: String,
}



fn default_option_true() -> Option<bool> { Some(true) }


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateOrderReq {
    pub input_mint: String,
    pub output_mint: String,
    pub maker: String,
    pub payer: String,
    pub params: CreateOrderParams,
    pub compute_unit_price: Option<String>,
    pub fee_account: Option<String>,
    #[serde(default = "default_option_true")]
    pub wrap_and_unwrap_sol: Option<bool>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CreateOrderRes {
    pub request_id: String,
    pub transaction: String,
    pub order: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelOrderReq {
    pub maker: String,
    pub order: String,
    // In microlamports, defaults to 95th percentile of priority fees
    pub compute_unit_price: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelOrderRes {
    pub request_id: String,
    pub transaction: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelOrdersReq {
    pub maker: String,
    pub orders: Vec<String>,
    // In microlamports, defaults to 95th percentile of priority fees
    pub compute_unit_price: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CancelOrdersRes {
    pub request_id: String,
    pub transaction: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetTriggerOrdersReq {
    pub user: String,
    pub order_status: String,
    pub input_mint: Option<String>,
    pub output_mint: Option<String>,
    pub include_failed_tx: Option<bool>,
    pub page: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetTriggerOrdersRes {
    pub user: String,
    pub order_status: String,
    pub orders: Vec<TriggerOrder>,
    pub total_pages: u64,
    pub page: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct TriggerOrder {
    pub user_pubkey: String,
    pub order_key: String,
    pub input_mint: String,
    pub output_mint: String,
    pub making_amoutn: String,
    pub taking_amount: String,
    pub remaining_making_amount: String,
    pub remaining_taking_amount: String,
    pub raw_making_amount: String,
    pub raw_taking_amount: String,
    pub raw_remaining_making_amount: String,
    pub raw_remaining_taking_amount: String,
    pub slippage_bps: u32,
    pub expired_at: String,
    pub updated_at: String,
    pub status: String,
    pub open_tx: String,
    pub close_tx: String,
    pub program_version: String,
    pub trades: Vec<Trade>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Trade {
    pub order_key: String,
    pub keeper: String,
    pub input_mint: String,
    pub output_mint: String,
    pub input_amount: String,
    pub output_amount: String,
    pub raw_input_amount: String,
    pub raw_output_amount: String,
    pub fee_mint: String,
    pub fee_amount: String,
    pub raw_fee_amount: String,
    pub tx_id: String,
    pub confirmed_at: String,
    pub action: String,
    pub product_meta: Option<String>,
}