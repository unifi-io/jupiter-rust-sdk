use crate::{token::{GetByCategoryQueryReq, GetByCategoryQueryRes, GetByTagQueryReq, GetByTagQueryRes, GetRecentRes, TokenSearchReq, TokenSearchRes}, JupiterClient, JupiterError};




#[derive(Clone)]
pub struct TokenService<'a>{
    client: &'a JupiterClient,
}


impl<'a> TokenService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }

    pub async fn search(
        &self,
        req: &TokenSearchReq,
    ) -> Result<TokenSearchRes, JupiterError> {
        let path = "/tokens/v2/search";
        self.client.get_json_with_query(&path, req).await
    }

    pub async fn get_by_tag(
        &self,
        req: &GetByTagQueryReq,
    ) -> Result<GetByTagQueryRes, JupiterError> {
        let path = "/tokens/v2/tag";
        self.client.get_json_with_query(&path, req).await
    }

    pub async fn get_by_category(
        &self,
        category: String,
        // 5m, 1h, 6h, 24h
        interval: String,
        req: &GetByCategoryQueryReq,
    ) -> Result<GetByCategoryQueryRes, JupiterError> {
        let path = format!("/tokens/v2/{}/{}", category, interval);
        self.client.get_json_with_query(&path, req).await
    }

    pub async fn recent(&self) -> Result<GetRecentRes, JupiterError> {
        let path = "/tokens/v2/recent";
        self.client.get_json(&path).await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_search() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let token_service = TokenService::new(&client);

        let res = token_service.search(&TokenSearchReq {
            query: "trump".to_string(),
        }).await;

        match res {
            Ok(res) => {
                // assert_eq!(res.error_code, None);
                println!("token_search: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("token_search error: {}", e);
            }
        }
    }
}

