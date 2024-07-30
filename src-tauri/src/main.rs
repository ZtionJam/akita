#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod domain;
mod action;
mod utils;
mod qwen;

use tauri::Manager;
use window_shadows::set_shadow;


fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            let _ = main_window.set_focus();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![action::send_ques,action::setting,action::get_config,action::set_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
