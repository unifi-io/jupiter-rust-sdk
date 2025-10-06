use serde::{Deserialize, Serialize};
use std::collections::HashMap;




#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccount {
    pub account: String,
    pub amount: String,
    pub ui_amount: f64,
    pub ui_amount_string: String,
    pub is_frozen: bool,
    pub is_associated_token_account: bool,
    pub decimals: u8,
    pub program_id: String,
    pub exclude_from_net_worth: bool,
}





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldingsRes {
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    pub amount: String,
    pub ui_amount: f64,
    pub ui_amount_string: String,
    pub tokens: HashMap<String, Vec<TokenAccount>>,
}