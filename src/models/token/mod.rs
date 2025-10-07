use serde::{Deserialize, Serialize};
use serde_with::serde_as;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct TokenSearchReq {
    pub query: String,
}


pub type TokenSearchRes = Vec<Token>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Token {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub icon: String,
    pub decimals: i32,
    pub twitter: String,
    pub telegram: String,
    pub website: String,
    pub dev: String,
    pub circ_supply: f64,
    pub total_supply: f64,
    pub token_program: String,
    pub launchpad: String,
    pub partner_config: String,
    pub graduated_pool: String,
    pub graduated_at: String,
    pub holder_count: i64,
    pub fdv: f64,
    pub mcap: f64,
    pub usd_price: f64,
    pub price_block_id: i64,
    pub liquidity: LiquidityStat,
    pub organic_score: f64,
    // high, medium, low
    pub organic_score_label: String,
    pub is_verified: bool,
    pub cexes: Vec<String>,
    pub tags: Vec<String>,
    pub updated_at: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct LiquidityStat {
    pub stats5m: LiquidityStatItem,
    pub stats1h: LiquidityStatItem,
    pub stats6h: LiquidityStatItem,
    pub stats24h: LiquidityStatItem,
    pub firt_pool: Pool,
    pub audit: Audit,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Pool {
    pub id: String,
    pub created_at: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Audit {
    pub is_sus: bool,
    pub mint_authority_disabled: bool,
    pub freeze_authority_disabled: bool,
    pub top_holders_percentage: f64,
    pub dev_balance_percentage: f64,
    pub dev_migrations: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct LiquidityStatItem {
    pub price_cange: String,
    pub holder_change: String,
    pub liquidity_change: String,
    pub volume_change: String,
    pub buy_volume: String,
    pub sell_volume: String,
    pub buy_organic_volume: String,
    pub sell_organic_volume: String,
    pub num_buys: String,
    pub num_sells: String,
    pub num_traders: String,
    pub num_organic_buyers: String,
    pub num_net_buyers: String,
}





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetByTagQueryReq {
    pub query: String,
}


pub type GetByTagQueryRes = Vec<Token>;



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetByCategoryQueryReq {
    pub limit: i64,
}

pub type GetByCategoryQueryRes = Vec<Token>;


pub type GetRecentRes = Vec<Token>;