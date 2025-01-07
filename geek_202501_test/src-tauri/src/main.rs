use std::process::Command;
use tauri::http::{header, response};
use serde_json::{json, Value};
use dotenv::dotenv;
use std::env;
mod func;
use func::chrome;

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

#[tauri::command]
fn open_slack_channel() -> Result<(), String> {
    let slack_team_id = env::var("SLACK_TEAM_ID").unwrap_or_else(|_| "default_url".to_string());
    let slack_api_id = env::var("SLACK_API_ID").unwrap_or_else(|_| "default_url".to_string());
    let slack_url = format!("slack://channel?team={}&id={}", slack_team_id, slack_api_id);
    println!("{}", &slack_url);
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

#[tauri::command]
async fn open_notion() -> Result<(), String> {
    let PARENT_PAGE_ID = env::var("PARENT_PAGE_ID").unwrap_or_else(|_| "default_url".to_string());
    let authorization_key = env::var("AUTHORIZATION_KEY").unwrap_or_else(|_| "default_url".to_string());

    println!("{} {}",PARENT_PAGE_ID, authorization_key);

    let data = json!({
        "parent": {"type": "page_id", "page_id": PARENT_PAGE_ID},
        "properties": {
            "title": [
                {
                    "type": "text",
                    "text": {
                        "content": "Child Page Title"
                    }
                }
            ]
        }
    });
    if cfg!(target_os = "macos") {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.notion.com/v1/pages")
            .header("Authorization", authorization_key)
            .header("Content-Type", "application/json")
            .header("Notion-Version", "2022-06-28")
            .json(&data)
            .send()
            .await
            .map_err(|e| e.to_string())?;

            println!("{:?}",&response.headers());
            // let json_data = &response.text().await?;
            let json_data: Value = response.json().await.map_err(|e| e.to_string())?;
            println!("{:#?}", json_data["id"]);

            // レスポンスヘッダーを取得
            if let Some(request_url) = json_data["url"].as_str() {
                // ヘッダー値を文字列として取得
                let request_url = request_url.replace("https://", "notion://");
                let _ = Command::new("open").arg(request_url).output();
            } else {
                println!("id が見つかりません");
            }
    } else if cfg!(target_os = "windows") {
        // Command::new("cmd").arg("/C").arg("start").arg(slack_url).output()
        return Err("Unsupported OS, use the MacOS".to_string());
    } else if cfg!(target_os = "linux") {
        // Command::new("xdg-open").arg(slack_url).output()
        return Err("Unsupported OS, use the MacOS".to_string());
    } else {
        return Err("Unsupported OS".to_string());
    };

    Ok(())
}

#[tauri::command]
async fn open_chrome_demo() -> Result<(), String> {
    return chrome::open_chrome_demo().await.map_err(|e| e.to_string());
    // if let Some(res) = chrome::open_chrome_demo() {
    //     Ok(())
    // } else {
    //     println!("err");
    //     Ok(())
    // }
}

#[tauri::command]
async fn store_notion_api() -> Result<(), String> {
    let email = "test@gmail.com";
    return chrome::store_notion_api(email.to_string()).await.map_err(|e| e.to_string());
}

fn main() {
    // .env ファイルを読み込む
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_slack_app,
            open_slack_channel,
            open_notion,
            open_chrome_demo,
            store_notion_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
