use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_with::{serde_as};




#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetPriceReq {
    pub ids: String,
}


pub type GetPriceRes = HashMap<String, PriceItem>;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriceItem {
    pub block_id: u64,
    pub decimals: i32,
    pub usd_price: f64,
    pub price_change_24h: f64,
}