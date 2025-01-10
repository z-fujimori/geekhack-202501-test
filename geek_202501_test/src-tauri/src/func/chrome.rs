use tauri::{State, Window, Emitter};
use tokio::sync::Notify;
use headless_chrome::{protocol::cdp::Page::{self, Viewport}, Browser};
use anyhow::Result;

use crate::InputState;

pub(crate) async fn open_chrome_demo() -> Result<()> {
        // ヘッドレスブラウザのインスタンスを作成
        let browser = Browser::default()?;
        // 新しいタブを開く
        let tab = browser.new_tab()?;
        
        // Googleの検索ページを開く
        tab.navigate_to("https://www.google.com")?
            .wait_until_navigated()?;
        println!("ページ開けた");

        // // ページのHTMLコンテンツを取得
        // let content = tab.get_content()?;
        // println!("ページコンテンツの一部: {}", &content[:100]); // 最初の100文字だけ表示
        if let Ok(_) = tab.find_element("textarea[name='q']") {
            println!("検索ボックスが見つかりました。ページ遷移成功");
        } else {
            println!("検索ボックスが見つかりませんでした。ページ遷移失敗");
        }
    
        // 検索ボックスを見つけて入力
        match tab.find_element("textarea[name=\"q\"]") {
            Ok(input_element) => {
                input_element.click()?.type_into("Rust programming language")?;
                println!("OK: 文字を入力しました");
            }
            Err(e) => {
                eprintln!("検索ボックスの入力に失敗しました: {}", e);

                let viewport = Some(Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: 1920.0,
                    height: 1080.0,
                    scale: 1.0,
                });
                // デバッグ用スクリーンショットを保存
                let screenshot_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, viewport, true)?;
                std::fs::write("debug_img/open_chrome_demo-debug_screenshot.png", screenshot_data)?;
                println!("スクリーンショットを保存しました: debug_screenshot.png");
            }
        }
        println!("入力後");

        // スクリーンショットを保存（オプション）
        let viewport = Some(Viewport {
            x: 0.0,
            y: 0.0,
            width: 1920.0,
            height: 1080.0,
            scale: 1.0,
        });
        let screenshot_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, viewport, true)?;
        std::fs::write("debug_img/open_chrome_demo-s.png", screenshot_data)?;

        // 検索ボタンを押す
        tab.wait_for_element("input[name='btnK']")?.click()?;

        // ページがロードされるまで待機
        let _ = tab.wait_for_element("body")?;

        // 検索結果のタイトルを取得
        let title = tab.get_title().map_err(|e| e.to_string());
        println!("Page title: {:?}", title);
    
        // スクリーンショットを保存（オプション）
        let viewport = Some(Viewport {
            x: 0.0,
            y: 0.0,
            width: 1920.0,
            height: 1080.0,
            scale: 1.0,
        });
        let screenshot_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, viewport, true)?;
        std::fs::write("debug_img/open_chrome_demo-screenshot.png", screenshot_data)?;
    
        Ok(())
}

pub(crate) async fn store_notion_api(
    email: String,
    window: Window,
    input_state: State<'_, InputState>, // `State` ラッパーを使用
) -> Result<()> {
    println!("{}",email);
    // ヘッドレスブラウザのインスタンスを作成
    // let browser = Browser::default()?;
    let browser = Browser::new(
        headless_chrome::LaunchOptions::default_builder()
            .headless(false) // ヘッドレスモードをオフ
            .build()
            .unwrap(),
    )?;
    // 新しいタブを開く
    let tab = browser.new_tab()?;
    // urlからページを開く
    tab.navigate_to("https://www.notion.so/profile/integrations")?.wait_until_navigated()?;
    // // コンテンツ確認
    // let content = tab.get_content()?;
    // println!("ページコンテンツの一部: {}", &content);
    // email入力
    tab.find_element("input[id='notion-email-input-2']")?.click()?.type_into(&email)?;

    // スクリーンショットを保存
    let viewport = Some(Viewport {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1080.0,
        scale: 1.0,
    });
    let screenshot_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, viewport, true)?;
    std::fs::write("debug_img/store_notion_api-screenshot1.png", screenshot_data)?;

    // 「続行」をクリック
    tab.find_element("form div[role='button'][aria-disabled='false']")?.click()?;
    // tab.find_element("div[aria-label='ヘルプ']")?.click()?;
    // // ページ遷移を待つ
    // tab.wait_for_element("input[id='notion-password-input-1']")?;
    let _ = tab.wait_until_navigated()?;
    // match tab.find_element("input[id='notion-password-input-1']") {
    //     Ok(_) => println!("要素が見つかりました"),
    //     Err(_) => println!("要素が見つかりませんでした"),
    // }
    
    // スクリーンショットを保存
    let viewport = Some(Viewport {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1080.0,
        scale: 1.0,
    });
    let screenshot_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, viewport, true)?;
    std::fs::write("debug_img/store_notion_api-screenshot2.png", screenshot_data)?;
    println!("第一段階終わり");

    {
        let mut sended = input_state.sended.lock().unwrap();
        *sended = true; // 一時停止フラグを有効化
        // input_state.sended = Arc::new(Mutex::new(true));
    }
    // event名は大文字NG
    window
        .emit("input_state", true)
        .map_err(|e| format!("Failed to emit progress: {}", e));
    println!("フロントへ");
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

    let onetime_code: String;
    {    
        onetime_code = {
            let pass = input_state.pass.lock().unwrap();
            pass.clone()
        };
        println!("ワンタイムコード, {}", onetime_code);
    }

    tab.find_element("input[id='notion-password-input-1']")?.click()?.type_into(&onetime_code)?;
    tab.find_element("form div[role='button'][aria-disabled='false']")?.click()?;
    let _ = tab.wait_until_navigated()?;
    tab.navigate_to("https://www.notion.so/profile/integrations/form/new-integration")?.wait_until_navigated()?;

    {
        let mut sended = input_state.sended.lock().unwrap();
        *sended = true; // 一時停止フラグを有効化
        // input_state.sended = Arc::new(Mutex::new(true));
    }
    // window
    //     .emit("input_state", true)
    //     .map_err(|e| format!("Failed to emit progress: {}", e));
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

    Ok(())
}

pub(crate) fn send_logincode_to_notion(login_code: String, input_state: State<'_, InputState>) {
    let mut sended = input_state.sended.lock().unwrap();
    *sended = false; // 一時停止フラグを無効化
    let mut pass = input_state.pass.lock().unwrap();
    *pass = login_code; // loginコードを共有
    input_state.notify.notify_one(); // 再開を通知
}
