// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// fn main() {
//     geek_202501_test_lib::run()
// }

// use tauri::api::process::Command;
// use tauri::api::process::CommandEvent;
use std::process::Command;

#[tauri::command]
fn open_slack_app() -> Result<(), String> {
    let app_path = if cfg!(target_os = "macos") {
        "/Applications/Slack.app/Contents/MacOS/Slack"
    } else if cfg!(target_os = "windows") {
        "C:\\Program Files\\Slack\\slack.exe"
    } else if cfg!(target_os = "linux") {
        "slack" // Linux では PATH に追加されていることを想定
    } else {
        return Err("Unsupported OS".to_string());
    };

    // Slack アプリを起動
    match Command::new(app_path).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to open Slack: {}", err)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_slack_app])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
