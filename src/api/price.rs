use crate::{price::{GetPriceReq, GetPriceRes}, JupiterClient, JupiterError};




#[derive(Clone)]
pub struct PriceService<'a>{
    client: &'a JupiterClient,
}


impl<'a> PriceService<'a> {
    pub fn new(client: &'a JupiterClient) -> Self {
        Self { client }
    }

    pub async fn get_price(
        &self,
        req: &GetPriceReq,
    ) -> Result<GetPriceRes, JupiterError> {
        let path = "/price/v3";
        self.client.get_json_with_query(&path, req).await
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_price() {
        let client = JupiterClient::new(crate::JupiterConfig::default()).unwrap();
        let price_service = PriceService::new(&client);

        let res = price_service.get_price(&&GetPriceReq {
            ids: "So11111111111111111111111111111111111111112,EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        }).await;

        match res {
            Ok(res) => {
                println!("get_price: {}", serde_json::to_string_pretty(&res).unwrap());
            }
            Err(e) => {
                panic!("get_price error: {}", e);
            }
        }
    }
}

