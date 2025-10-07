use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::lend::Instruction;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub enum SwapMode {
    ExactIn,
    ExactOut,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SwapQuoteReq {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: String,
    pub slippage_bps: u16,
    pub swap_mode: SwapMode,
    pub dexes: Vec<String>,
    pub exclude_dexes: Vec<String>,
    pub restrict_intermediate_tokens: bool,
    pub only_direct_routes: bool,
    pub as_legacy_transaction: bool,
    pub platform_fee_bps: u16,
    pub max_accounts: u64,
    pub dynamic_slippage: bool,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SwapQuoteRes {
    pub input_mint: String,
    pub in_amount: String,
    pub output_mint: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: SwapMode,
    pub slippage_bps: u16,
    pub platform_fee: PlatformFee,
    pub price_impact_pct: String,
    pub route_plan: Vec<RoutePlan>,
    pub context_slot: u64,
    pub time_taken: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PlatformFee {
    pub amount: String,
    pub fee_bps: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: u8,
    pub bps: u16,
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
pub struct SwapReq {
    pub user_public_key: String,
    pub payer: String,
    pub wrap_and_unwrap_sol: bool,
    pub use_shared_accounts: bool,
    pub fee_account: Option<String>,
    pub tracking_account: Option<String>,
    pub prioritization_fee_lamports: PrioritizationFeeLamports,
    pub as_legacy_transaction: bool,
    pub destination_token_account: String,
    pub dynamic_compute_unit_limit: bool,
    pub skip_user_accounts_rpc_calls: bool,
    pub dynamic_slippage: bool,
    pub compute_unit_price_micro_lamports: u64,
    pub blockhash_slots_to_expiry: u64,
    pub quote_response: SwapQuoteRes,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SwapRes {

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PrioritizationFeeLamports {
    pub priority_level_with_max_lamports: PriorityLevelWithMaxLamports,
    pub jito_tip_lamports: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PriorityLevelWithMaxLamports {
    pub priority_level: String,
    pub max_lamports: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SwapInstructionsRes {
    pub other_instructions: Vec<Instruction>,
    pub compute_budget_instructions: Vec<Instruction>,
    pub setup_instructions: Vec<Instruction>,
    pub swap_instruction: Instruction,
    pub cleanup_instruction: Instruction,
    pub address_lookup_table_addresses: Vec<String>,
}