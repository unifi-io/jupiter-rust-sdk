use std::time::Duration;
use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};

use crate::models::{
    order::OrderReq,
    order::OrderRes,
};
use crate::JupiterError;
use reqwest::{Client, Url};
use reqwest::header::{HeaderMap, CONTENT_TYPE};




const BODY_SNIPPET_LIMIT: usize = 4096;

fn body_excerpt(bytes: &[u8]) -> String {
    let s = String::from_utf8_lossy(bytes);
    if s.len() > BODY_SNIPPET_LIMIT {
        format!("{}…", &s[..BODY_SNIPPET_LIMIT])
    } else {
        s.to_string()
    }
}




#[derive(Clone, Debug)]
pub struct JupiterConfig {
    pub base_url: Url,
    pub timeout: Duration,
    pub user_agent: Option<String>,
}

impl Default for JupiterConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse("https://lite-api.jup.ag").unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: Some(format!("jupiter-rs/{}", env!("CARGO_PKG_VERSION"))),
        }
    }
}

impl JupiterConfig {
    pub fn builder() -> JupiterConfigBuilder {
        JupiterConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct JupiterConfigBuilder {
    base_url: Option<Url>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl JupiterConfigBuilder {
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(Url::parse(url).expect("valid base url"));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    pub fn build(self) -> JupiterConfig {
        JupiterConfig {
            base_url: self.base_url.unwrap_or_else(|| Url::parse("https://lite-api.jup.ag").unwrap()),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            user_agent: self.user_agent.or_else(|| Some(format!("jupiter-rs/{}", env!("CARGO_PKG_VERSION")))),
        }
    }
}

#[derive(Clone)]
pub struct JupiterClient {
    config: JupiterConfig,
    client: Client,
}

impl JupiterClient {
    pub fn new(config: JupiterConfig) -> Result<Self, JupiterError> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut builder = Client::builder()
            .default_headers(headers)
            .timeout(config.timeout);

        if let Some(ua) = &config.user_agent {
            builder = builder.user_agent(ua.clone());
        }

        let client = builder
            .build()
            .map_err(|e| JupiterError::Network(format!("failed to build http client: {e}")))?;

        Ok(Self { config, client })
    }

    #[inline]
    fn build_url(&self, path: &str) -> Result<Url, JupiterError> {
        self.config
            .base_url
            .join(path)
            .map_err(|e| JupiterError::Internal(format!("join url error: {e}")))
    }

    async fn parse_json<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, JupiterError> {
        let status = resp.status();
        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let bytes = resp.bytes().await.map_err(JupiterError::from)?;

        if !status.is_success() {
            return Err(JupiterError::Http {
                status: status.as_u16(),
                body: body_excerpt(&bytes),
                content_type,
            });
        }

        // 使用 serde_path_to_error 捕获精确路径
        let mut de = serde_json::Deserializer::from_slice(&bytes);
        match serde_path_to_error::deserialize::<_, T>(&mut de) {
            Ok(v) => Ok(v),
            Err(err) => {
                let path = err.path().to_string();
                let message = err.inner().to_string();
                Err(JupiterError::Parse {
                    message,
                    path,
                    body: body_excerpt(&bytes),
                })
            }
        }
    }

    pub(super) async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, JupiterError> {
        let url = self.build_url(path)?;
        let resp = self.client.get(url).send().await?;
        Self::parse_json(resp).await
    }

    pub(super) async fn get_json_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, JupiterError>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let url = self.build_url(path)?;
        let resp = self.client.get(url).query(query).send().await?;
        Self::parse_json(resp).await
    }

    #[allow(dead_code)]
    pub async fn post<T: for<'a> Deserialize<'a>, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, JupiterError> {
        let url = self.build_url(path)?;

        let resp = self.client.post(url)
            .json(body)
            .send()
            .await
            .map_err(JupiterError::from)?;

        Self::parse_json(resp).await
    }

    // ---------- Public APIs ----------

    pub async fn get_order(&self, req: &OrderReq) -> Result<OrderRes, JupiterError> {
        let path = "/ultra/v1/order";
        self.get_json_with_query(&path, req,).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_order() {
        let client = JupiterClient::new(JupiterConfig::default()).unwrap();
        let res = client.get_order(&OrderReq {
            input_mint: "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".to_string(),
            output_mint: "So11111111111111111111111111111111111111112".to_string(),
            amount: "500".to_string(),
            taker: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
            referral_account: None,
            referral_fee: None,
            exclude_dexes: None,
            exclude_routers: None,
        }).await.unwrap();

        assert_eq!(res.error_code, None);
        println!("order: {}", serde_json::to_string_pretty(&res).unwrap());
    }
}