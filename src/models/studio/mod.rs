use serde::{Deserialize, Serialize};
use serde_with::serde_as;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct LockedVestingParam {
    pub total_locked_vesting_amount: f64,
    pub cliff_unlock_amount: f64,
    pub number_of_vesting_period: f64,
    pub total_vesting_duration: i64,
    pub cliff_duration_from_migration_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct BuildCurveByMarketCapParam {
    pub quote_mint: String,
    pub initial_market_cap: f64,
    pub migration_market_cap: f64,
    pub token_quote_decimal: i64,
    pub locked_vesting_param: LockedVestingParam,

}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Fee {
    pub total_duration: i64,
    pub base_fee_mode: String,
    pub fee_bps: f64,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PoolCreateTxReq {
    pub build_curve_by_market_cap_param: BuildCurveByMarketCapParam,
    pub fee: Fee,
    pub is_lp_locked: bool,
    pub token_name: String,
    pub token_symbol: String,
    pub token_image_content_type: String,
    // Creator wallet public key
    pub creator: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PoolCreateTxRes {
    pub transaction: String,
    pub mint: String,
    pub image_presigned_url: String,
    pub metadata_presigned_url: String,
    pub image_url: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PoolSubmitReq {
    pub transaction: String,
    pub owner: String,
    pub content: String,
    pub header_image: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PoolSubmitRes {
    pub success: bool,
    pub data: PoolSubmitData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PoolSubmitData {
    pub mint: String,
    pub config_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetPoolByMintRes {
    pub success: bool,
    pub data: PoolByMintData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PoolByMintData {
    pub dammv2_pool_address: String,
    pub dbc_pool_address: String,
    pub config_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetDbcFeeReq {
    pub pool_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetDbcFeeRes {
    pub unclaimed: String,
    pub total: String,    
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DbcFeeCreateTxReq {
    pub owner_wallet: String,
    pub pool_address: String,
    pub max_quote_amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DbcFeeCreateTxRes {
    pub transaction: String,
}