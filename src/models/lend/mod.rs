use serde::{Deserialize, Serialize};
use serde_with::serde_as;






#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct LendReq {
    pub asset: String,
    pub signer: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct LendRes {
    pub transaction: String,
}
