use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use tauri::{Manager, State, Window, Emitter};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
struct AppState {
    paused: Arc<Mutex<bool>>,
    notify: Arc<Notify>,
}

#[tauri::command]
async fn start_processing(
    window: Window,
    state: State<'_, AppState>, // `State` ラッパーを使用
) -> Result<(), String> {
    for i in 0..5 {
        // 処理の進行状況をフロントエンドに送信
        window
            .emit("pro", format!("Step {} started", i + 1))
            .map_err(|e| format!("Failed to emit progress: {}", e))?;

        // 一時停止チェック
        loop {
            let paused = {
                let paused_lock = state.paused.lock().unwrap();
                *paused_lock
            };
            if !paused {
                break;
            }
            println!("Paused... waiting for resume");
            state.notify.notified().await; // 再開を待機
        }

        // 模擬的な処理（1秒間スリープ）
        println!("Processing step {}", i + 1);
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
#[tauri::command]
fn pause_processing(state: State<'_, AppState>) {
    let mut paused = state.paused.lock().unwrap();
    *paused = true; // 一時停止フラグを有効化
}
#[tauri::command]
fn resume_processing(state: State<'_, AppState>) {
    let mut paused = state.paused.lock().unwrap();
    *paused = false; // 一時停止フラグを無効化
    state.notify.notify_one(); // 再開を通知
}

#[derive(Clone)]
struct InputState {
    sended: Arc<Mutex<bool>>,
    // pass: Arc<Mutex<String>>,
    notify: Arc<Notify>,
}

#[tauri::command]
async fn input_two_word(
    window: Window,
    input_state: State<'_, InputState> // Stateラッパーを使用,,,らしい 「' (アポストロフィ)」でライフタイムを指定？
) -> Result<(), String> {
    println!("前段階の処理 開始\n~~~ --- ~~~ ---");
    {
        let mut sended = input_state.sended.lock().unwrap();
        *sended = true; // 一時停止フラグを有効化
        // input_state.sended = Arc::new(Mutex::new(true));
    }

    window
        .emit("input_state", "ture")
        .map_err(|e| format!("Failed to emit progress: {}", e))?;
    // 一時停止チェック
    loop {
        let sended = {
            let sended_lock = input_state.sended.lock().unwrap();
            *sended_lock
        };
        if !sended {
            break;
        }
        println!("Sended... waiting for resume");
        input_state.notify.notified().await; // 再開を待機
    }
    println!("2の処理 開始\n~~~ --- ~~~ ---\n終了\n");
    Ok(())
}
#[tauri::command]
fn resume_input(input_state: State<'_, InputState>) {
    let mut sended = input_state.sended.lock().unwrap();
    *sended = false; // 一時停止フラグを無効化
    input_state.notify.notify_one(); // 再開を通知
}


fn main() {
    let state = AppState {
        paused: Arc::new(Mutex::new(false)),
        notify: Arc::new(Notify::new()),
    };
    let input_state = InputState {
        sended:  Arc::new(Mutex::new(false)),
        notify: Arc::new(Notify::new()),
    };

    tauri::Builder::default()
        .manage(state) // グローバル状態として共有
        .manage(input_state) 
        .invoke_handler(tauri::generate_handler![
            start_processing,
            pause_processing,
            resume_processing,
            input_two_word,
            resume_input,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}