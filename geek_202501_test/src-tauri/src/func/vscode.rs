use std::process::Command;
use anyhow::Result;

pub(crate) fn open_vscode() -> Result<()> {
    println!("eee");
    Command::new("open").arg("-a").arg("visual studio code").output()?;

    // let script = r#"
    // tell application "Visual Studio Code"
    //     activate
    // end tell
    // "#;

    // Command::new("osascript")
    //     .arg("-e")
    //     .arg(script)
    //     .output()?;

    Ok(())
}

pub(crate) fn toggle_word() -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(r#"
        tell application "System Events"
            set windowList to {}
            set appList to application processes
            repeat with appProc in appList
                set appName to name of appProc
                try
                    set winList to windows of appProc
                    repeat with win in winList
                        set winName to name of win
                        set end of windowList to appName & ": " & winName
                    end repeat
                end try
            end repeat
        end tell
        
        return windowList
        "#)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
    // Ok(())
}
