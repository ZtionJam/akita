use core::panic;

use std::time::Duration;

use reqwest::Client;
use tauri::AppHandle;

use crate::domain::MessageChunk;
use crate::domain::MessageReq;
use crate::utils;
use crate::utils::*;

#[tauri::command]
pub async fn send_ques(req: MessageReq, app: AppHandle) {
    println!("{:?}", req);
    let url = "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation";
    let qwen_req = gen_qwen_req_from_front_req(&req);

    let client = Client::builder()
        .timeout(Duration::from_secs(3000))
        .build()
        .unwrap();

    let mut response = match client
        .post(url)
        .body(qwen_req)
        .header(
            "Authorization",
            "Bearer sk-69491a35e6e64f8f8dbaf913b26e62e3",
        )
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
        if let Some(data) = chunk {
            let str = &String::from_utf8_lossy(&data)
                .to_string()
                .trim()
                .to_string();
            if let Some(qwen_chunk) = utils::resolve_qwen_sse_chunk(str) {
                let msg_chunk = MessageChunk {
                    over: qwen_chunk.output.finish_reason.eq("stop"),
                    chunk_content: qwen_chunk.output.text,
                    msg_id: req.ans_id,
                };
                utils::send_msg_chunk(msg_chunk, &app);
            }
        }
    }
}
