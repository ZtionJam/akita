
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReq {
    pub msg_history: Vec<MsgFront>,
    pub ans_id: u64,
    pub model_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgFront {
    pub id: u64,
    pub role: String,
    pub content: String,
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageChunk {
    pub over: bool,
    pub chunk_content: String,
    pub msg_id: u64,
}
