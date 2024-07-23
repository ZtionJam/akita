use tauri::{AppHandle, Manager};

use crate::domain::{MessageChunk, MessageReq};
use crate::qwen::{QwenChunk, QwenInput, QwenMessage, QwenParameters, QwenReq};

pub fn resolve_qwen_sse_chunk(str: &String) -> Option<QwenChunk> {
    for line in str.lines() {
        if line.starts_with("data:") {
            let line = line.replace("data:", "");
            if let Ok(chunk) = serde_json::from_str(line.as_str()) {
                return Some(chunk);
            }
        }
    }
    None
}

pub fn gen_qwen_req_from_front_req(req: &MessageReq) -> QwenReq {
    QwenReq {
        model: req.model_name.clone(),
        input: QwenInput {
            messages: req
                .msg_history
                .iter()
                .map(|msg| QwenMessage {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                })
                .collect(),
        },
        parameters: QwenParameters {
            incremental_output: true,
        },
    }
}

pub fn send_msg_chunk(chunk: MessageChunk, app: &AppHandle) {
    let _ = app.emit_all("msg_chunk", chunk);
}
