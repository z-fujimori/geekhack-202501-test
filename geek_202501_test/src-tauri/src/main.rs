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
// #[tauri::command]
// fn open_slack_channel() -> Result<(), String> {
//     // Slack URLスキーム
//     let slack_url = "slack://channel?team=T074C4CN316&id=C075QS804BD";

//     // システムのデフォルトブラウザまたはSlackアプリでURLを開く
//     // match tauri::shell::open("slack", slack_url, None) {
//     match tauri::shell::open("slack", slack_url, None) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(format!("Failed to open Slack channel: {}", e)),
//     }
// }
#[tauri::command]
fn open_slack_channel() -> Result<(), String> {
    let slack_url = "slack://channel?team=T074C4CN316&id=C075QS804BD";

    // macOSの場合: `open`
    // Linuxの場合: `xdg-open`
    // Windowsの場合: `start`
    let command = if cfg!(target_os = "macos") {
        Command::new("open").arg(slack_url).output()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("start").arg(slack_url).output()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(slack_url).output()
    } else {
        return Err("Unsupported OS".to_string());
    };

    match command {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to open Slack channel: {}", err)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_slack_app,
            open_slack_channel,
            ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
