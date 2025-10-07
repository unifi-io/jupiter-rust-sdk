use crate::{lend::{GetEarningsReq, GetEarningsRes}, models::lend::{GetPositionsReq, GetPositionsRes}, JupiterClient, JupiterError};





#[derive(Clone)]
pub struct LendService<'a>{
    client: &'a JupiterClient,
}


// #[async_trait]
impl<'a> LendService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }


    pub async fn get_positions(
        &self,
        req: &GetPositionsReq,
    ) -> Result<GetPositionsRes, JupiterError> {
        let path = "/lend/v1/earn/positions";
        self.client.get_json_with_query(&path, req).await
    }

    pub async fn get_earnings(
        &self,
        req: &GetEarningsReq,
    ) -> Result<GetEarningsRes, JupiterError> {
        let path = "/lend/v1/earn/earnings";
        self.client.get_json_with_query(&path, req).await
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_get_positions() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let lend_service = LendService::new(&client);

        let res = lend_service.get_positions(&GetPositionsReq {
            users: "ANAUcDCU3Jfao3mtxBdttjEH7F3Ja7SyjGBKUa9Cruc5".to_string(),
        }).await;

        match res {
            Ok(res) => {
                // assert_eq!(res.error_code, None);
                println!("get_positions: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("get_positions error: {}", e);
            }
        }
    }


    #[tokio::test]
    async fn test_get_earnings() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let lend_service = LendService::new(&client);

        let res = lend_service.get_earnings(&GetEarningsReq {
            user: "ANAUcDCU3Jfao3mtxBdttjEH7F3Ja7SyjGBKUa9Cruc5".to_string(),
            positions: "9BEcn9aPEmhSPbPQeFGjidRiEKki46fVQDyPpSQXPA2D".to_string(),
        }).await;

        match res {
            Ok(res) => {
                // assert_eq!(res.error_code, None);
                println!("get_earnings: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("get_earnings error: {}", e);
            }
        }
    }
}

