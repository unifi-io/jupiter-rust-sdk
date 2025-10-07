use crate::{send::{CraftClawbackReq, CraftClawbackRes, CraftSendReq, CraftSendRes, InviteHistoryReq, InvitesPage, PendingInvitesReq}, JupiterClient, JupiterError};






#[derive(Clone)]
pub struct SendService<'a>{
    client: &'a JupiterClient,
}



impl<'a> SendService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }

    pub async fn craft_send(
        &self,
        req: &CraftSendReq,
    ) -> Result<CraftSendRes, JupiterError> {
        let path = "/send/v1/craft-send";
        self.client.post(&path, req).await
    }

    pub async fn craft_clawback(
        &self,
        req: &CraftClawbackReq,
    ) -> Result<CraftClawbackRes, JupiterError> {
        let path = "/send/v1/craft-send";
        self.client.post(&path, req).await
    }

    pub async fn pending_invites(
        &self,
        req: &PendingInvitesReq,
    ) -> Result<InvitesPage, JupiterError> {
        let path = "/send/v1/pending-invites";
        self.client.post(&path, req).await
    }

    pub async fn invite_history(
        &self,
        req: &InviteHistoryReq,
    ) -> Result<InvitesPage, JupiterError> {
        let path = "/send/v1/invite-history";
        self.client.post(&path, req).await
    }

}