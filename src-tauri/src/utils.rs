

use std::path::Path;
use std::fs;

use tauri::api::path::config_dir;
use tauri::{AppHandle, Manager};

use crate::domain::{AppConfig, MessageChunk, MessageReq};
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

pub fn set_config(config: &AppConfig){
    let config_dir =config_dir().unwrap();
    let config_path = config_dir.join("akita_config.json");

    let json_string = serde_json::to_string_pretty(config).unwrap();
    fs::write(&config_path, json_string).unwrap();
}


pub fn get_config() -> Result<AppConfig, ()> {
    let config_dir = config_dir().unwrap();
    let config_path = config_dir.join("akita_config.json");

    if !Path::new(&config_path).exists() {
        let default_config = AppConfig {
            api_key:"xxxxxxx".to_string()
        };
        set_config(&default_config);
        println!("create config:{:?}",config_path);
        return Ok(default_config);
    }

    let json_string = fs::read_to_string(config_path).unwrap();
    let config: AppConfig = serde_json::from_str(&json_string).unwrap();

    Ok(config)
}