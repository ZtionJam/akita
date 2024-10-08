use core::panic;

use std::time::Duration;

use reqwest::Client;
use tauri::AppHandle;
use tauri::Manager;
use tauri::WindowBuilder;
use tauri::WindowUrl;
use window_shadows::set_shadow;

use crate::domain::AppConfig;
use crate::domain::MessageChunk;
use crate::domain::MessageReq;
use crate::utils;
use crate::utils::*;

#[tauri::command]
pub async fn send_ques(req: MessageReq, app: AppHandle) {
    println!("{:?}", req);
    let url = "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation";
    let qwen_req = gen_qwen_req_from_front_req(&req);

    let app_config = utils::get_config().unwrap();

    let client = Client::builder()
        .timeout(Duration::from_secs(3000))
        .build()
        .unwrap();

    let mut response = match client
        .post(url)
        .body(qwen_req)
        .header("Authorization", app_config.api_key)
        .header("Content-Type", "application/json")
        .header("X-DashScope-SSE", "enable")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            panic!("err")
        }
    };

    while let Ok(chunk) = response.chunk().await {
        let mut finish = true;
        if let Some(data) = chunk {
            let str = &String::from_utf8_lossy(&data)
                .to_string()
                .trim()
                .to_string();
            if let Some(qwen_chunk) = utils::resolve_qwen_sse_chunk(str) {
                finish = qwen_chunk.output.finish_reason.eq("stop");
                let msg_chunk = MessageChunk {
                    over: finish,
                    chunk_content: qwen_chunk.output.text,
                    msg_id: req.ans_id,
                };
                utils::send_msg_chunk(msg_chunk, &app);
            } else {
                utils::send_msg_chunk(
                    MessageChunk {
                        over: true,
                        chunk_content: "请求失败，请检查ApiKey配置或网络情况".to_string(),
                        msg_id: req.ans_id,
                    },
                    &app,
                );
            }
        }
        if finish {
            break;
        };
    }
}

#[tauri::command]
pub async fn setting(handle: AppHandle) {
    //config
    let mut config = handle.config().tauri.windows.get(0).unwrap().clone();
    config.label = "setting".to_string();
    config.title = "设置".to_string();
    config.height = 350.0;
    config.width = 400.0;
    config.center = false;
    config.url = WindowUrl::App("/#/setting".parse().unwrap());

    let setting_window = match WindowBuilder::from_config(&handle, config).build() {
        Ok(w) => w,
        Err(e) => {
            if e.to_string().contains("exists") {
                if let Some(win) = handle.get_window("setting") {
                    let _ = win.set_focus();
                    return;
                }
            }
            panic!("open setting err")
        }
    };
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&setting_window, true).unwrap();
}

#[tauri::command]
pub fn get_config() -> Result<AppConfig, ()> {
    utils::get_config()
}

#[tauri::command]
pub fn set_config(config: AppConfig) {
    utils::set_config(&config);
}
