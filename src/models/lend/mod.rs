use serde::{Deserialize, Serialize};
use serde_with::serde_as;






#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DepositReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DepositRes {
    pub transaction: String,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct WithdrawReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct WithdrawRes {
    pub transaction: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct MintReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct MintRes {
    pub transaction: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct RedeemReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct RedeemRes {
    pub transaction: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DepositInstructionsReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct DepositInstructionsRes {
    pub program_id: Vec<String>,
    pub accounts: Vec<AccountInfo>,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct WithdrawInstructionsReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct WithdrawInstructionsRes {
    pub program_id: Vec<String>,
    pub accounts: Vec<AccountInfo>,
    pub data: String,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct MintInstructionsReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Instruction {
    pub program_id: Vec<String>,
    pub accounts: Vec<AccountInfo>,
    pub data: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct RedeemInstructionsReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct RedeemInstructionsRes {
    pub program_id: Vec<String>,
    pub accounts: Vec<AccountInfo>,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Asset {
    pub address: String,
    pub chain_id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub logo_uri: Option<String>,
    pub price: String,
    pub coingecko_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct LiquiditySupplyData {
    pub mode_with_interest: bool,
    pub supply: String,
    pub withdrawal_limit: String,
    pub last_update_timestamp: String,
    pub expand_percent: f64,
    pub expand_duration: String,
    pub base_withdrawal_limit: String,
    pub withdrawable_until_limit: String,
    pub withdrawable: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Token {
    pub id: u64,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub asset_address: String,
    pub asset: Asset,
    pub total_assets: String,
    pub total_supply: String,
    pub convert_to_shares: String,
    pub convert_to_assets: String,
    pub rewards_rate: String,
    pub supply_rate: String,
    pub total_rate: String,
    pub rebalance_difference: String,
    pub liquidity_supply_data: LiquiditySupplyData,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetPositionsReq {
    pub users: String,
}

pub type GetPositionsRes = Vec<Position>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Position {    
    pub token: Token,
    // Address of user's wallet
    pub owner_address: String,
    // Balance of user's jlTokens
    pub shares: String,
    // Balance of user's USDC in protocol (including intereste accured)
    pub underlying_assets: String,
    // Balance of user's USDC in wallet
    pub underlying_balance: String, 
    pub allowance: String,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetEarningsReq {
    pub user: String,
    pub positions: String,
}


pub type GetEarningsRes = Vec<GetEarningsResItem>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetEarningsResItem {
    pub address: String,
    pub owner_address: String,
    pub total_deposits: String,
    pub total_withdraws: String,
    pub total_balance: String,
    pub total_assets: String,
    pub earnings: String,
    pub slot: u64,
}