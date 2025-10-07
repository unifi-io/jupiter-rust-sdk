use serde::{Deserialize, Serialize};
use serde_with::serde_as;





#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CraftSendReq {
    pub invite_signer: String,
    pub sender: String,
    pub amount: String,
    pub mint: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CraftSendRes {
    pub tx: String,
    pub expiry: String,
    pub total_fee_lamports: String,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CraftClawbackReq {
    pub invite_pda: String,
    pub sender: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct CraftClawbackRes {
    pub tx: String,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct PendingInvitesReq {
    pub address: String,
    pub page: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct InvitesPage {
    pub invites: Vec<Invite>,
    pub has_more_data: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct Invite {
    pub invite_signer: String,
    pub invite_pda: String,
    pub sender: String,
    pub expiry: String,
    pub amount: String,
    pub creation_tx: String,
    pub deletion_tx: String,
    pub receiver: String,
    pub action: String,
    pub creation_time: String,
    pub deletion_time: String,
    pub mint: String,
    pub confirmed: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct InviteHistoryReq {
    pub address: String,
    pub page: i32,
}