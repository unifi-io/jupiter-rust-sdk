use serde::{Deserialize, Serialize};
use serde_with::serde_as;










#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct GetHoldingsRes {
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
}