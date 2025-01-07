use headless_chrome::{protocol::cdp::Page::{self, Viewport}, Browser};
use anyhow::Result;

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

pub(crate) async fn store_notion_api(email: String) -> Result<()> {
    // ヘッドレスブラウザのインスタンスを作成
    let browser = Browser::default()?;

    // 新しいタブを開く
    let tab = browser.new_tab()?;

    // urlからページを開く
    tab.navigate_to("https://www.notion.so/profile/integrations")?.wait_until_navigated()?;

    // スクリーンショットを保存
    let viewport = Some(Viewport {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1080.0,
        scale: 1.0,
    });
    let screenshot_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, viewport, true)?;
    std::fs::write("debug_img/store_notion_api-screenshot.png", screenshot_data)?;

    Ok(())
}


