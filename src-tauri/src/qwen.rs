use reqwest::Body;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QwenReq {
    pub model: String,
    pub input: QwenInput,
    pub parameters: QwenParameters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QwenInput {
    pub messages: Vec<QwenMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QwenMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QwenParameters {
    pub incremental_output: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QwenChunk {
    pub output: QwenOutput,
    pub usage: Usage,
    pub request_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QwenOutput {
    pub finish_reason: String,
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub output_tokens: u32,
    pub input_tokens: u32,
}

impl From<QwenReq> for Body {
    fn from(custom: QwenReq) -> Self {
        Body::from(serde_json::to_string(&custom).unwrap())
    }
}
