// import { invoke } from '@tauri-apps/api/tauri';

import { invoke } from "@tauri-apps/api/core";
import SimpleButton from "./components/SimpleButton";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

function App() {
  // const gmail = import.meta.env.VITE_APP_GMAIL;
  const [userEmail, setUserEmail] = useState("");
  const [userLoginCode, setUserLoginCode] = useState("");
  const [inputState, setInputState] = useState(false);

  useEffect(() => {
    // バックエンドからの進行状況を受け取る
    const unlisten = listen("input_state", (event) => {
      setInputState(event.payload as boolean);
    });
    // アンマウント時にリスナーを解除
    return () => {
      unlisten.then((unsub) => unsub());
    };
  }, []);

  const openSlackApp = async () => {
    try {
      await invoke('open_slack_app');
      console.log('Slack app opened successfully');
    } catch (error) {
      console.error('Failed to open Slack app:', error);
    }
  };
  async function openSlackWithCommand() {
    try {
      await invoke('open_slack_channel');
      console.log('Opened Slack successfully');
    } catch (error) {
      console.error('Failed to open Slack:', error);
    }
  }
  async function openNotion() {
    try {
      await invoke('open_notion');
      console.log("notion");
    } catch (error) {
      console.error('Failed to open Notion:', error);
    }
  }
  async function openChrom() {
    try {
      await invoke("open_chrome_demo");
      console.log("chrome demo");
    } catch (err) {
      console.error('Failed Chrome', err);
    }
  }
  async function storeNotionApi() {
    try {
      await invoke("store_notion_api", {"email": userEmail});
    } catch (err) {
      console.error("failed store_notion_api", err);
    }
  }
  async function sendOntimeLoginCode() {
    try {
      await invoke("send_logincode_to_notion", {"pass": userLoginCode});
    } catch (e) {
      console.error(e);
    }
  }
  async function vscodeWindowShow() {
    try {
      await invoke("show_window_data");
    } catch (e) {
      console.error(e);
    }
  }
  async function openVscode() {
    try {
      await invoke("open_vscode");
    } catch (e) {
      console.error(e);
    }
  }

  return (
    <div>
      <div>
        <SimpleButton text="VS Code" function={vscodeWindowShow} />
        <SimpleButton text="Open Code" function={openVscode} />
      </div>
      <div className="m-3">
        <h1>Notion API</h1>
        {inputState ?(
          <div>
            <input type="text" autoCapitalize="off" value={userLoginCode} onChange={(e)=>setUserLoginCode(e.target.value)} className="border border-gray-300 rounded-md" />
            <button onClick={sendOntimeLoginCode}>Send ワンタイムloginコード</button>
          </div>
        ) : (
          <div>
            <input type="text" autoCapitalize="off" value={userEmail} onChange={(e)=>setUserEmail(e.target.value)} className="border border-gray-300 rounded-md" />
            <button onClick={storeNotionApi}>Send Email</button>
          </div>
        )}
      </div>

      <div className="grid grid-cols-2 gap-3">
        <SimpleButton text="Slack(ずっと動く,,,)" function={openSlackApp} />
        <SimpleButton text="Slack" function={openSlackWithCommand} />
        <SimpleButton text="Notion make child page" function={openNotion}/>
        <SimpleButton text="Open Chrom" function={openChrom} />
      </div>
    </div>
  );
}

export default App;
