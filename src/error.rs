use thiserror::Error;


#[derive(Debug, Error)]
pub enum JupiterError {
    #[error("network error: {0}")]
    Network(String),

    #[error("http error: status={status}, content_type={content_type:?}, body={body}")]
    Http {
        status: u16,
        body: String,
        content_type: Option<String>,
    },

    #[error("parse error at {path}: {message}. body={body}")]
    Parse {
        message: String,
        path: String,
        body: String,
    },

    #[error("internal error: {0}")]
    Internal(String),
}

impl From<reqwest::Error> for JupiterError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            JupiterError::Network("timeout".into())
        } else {
            JupiterError::Network(err.to_string())
        }
    }
}

/*
impl From<serde_json::Error> for JupiterError {
    fn from(err: serde_json::Error) -> Self {
        JupiterError::Parse {
            message: err.to_string(),
            path: "".to_string(),
            body: "".to_string(),
        }
    }
}*/