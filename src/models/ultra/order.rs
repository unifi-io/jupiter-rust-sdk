use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::{F64, U128};






#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct OrderReq {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: String,
    pub taker: String,
    pub referral_account: Option<String>,
    pub referral_fee: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, String>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_routers: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, String>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_dexes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SwapInfo {
    pub amm_key: String,
    pub label: String,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,    
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]

pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: U128,
    pub bps: U128,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PlatformFee {
    pub amount: Option<String>,
    pub fee_bps: U128,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct OrderRes {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub slippage_bps: u16,
    pub in_usd_value: U128,
    pub out_usd_value: U128,
    pub price_impact: F64,
    pub swap_usd_value: U128,
    pub price_impact_pct: F64,
    pub route_plan: Vec<RoutePlan>,
    pub fee_mint: String,
    pub fee_bps: U128,
    pub signature_fee_lamports: U128,
    pub prioritization_fee_lamports: U128,
    pub rent_fee_lamports: U128,
    pub swap_type: String,
    pub router: String,
    pub transaction: Option<String>,
    pub gasless: bool,
    pub request_id: String,
    pub total_time: U128,
    pub taker: Option<String>,
    pub quote_id: Option<U128>,
    pub maker: Option<String>,
    pub expire_at: Option<String>,
    pub platform_fee: Option<PlatformFee>,
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
}
