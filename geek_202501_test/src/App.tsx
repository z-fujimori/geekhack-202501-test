// import { invoke } from '@tauri-apps/api/tauri';

import { invoke } from "@tauri-apps/api/core";
import SimpleButton from "./components/SimpleButton";

function App() {
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
      await invoke("store_notion_api", {"email": "taichi.2003.329@gmail.com"});
    } catch (err) {
      console.error("failed store_notion_api", err);
    }
  }

  return (
    <div>
      <SimpleButton text="Slack Desktop 押さん方がええよ(ずっと動く)" function={openSlackApp} />
      <SimpleButton text="Slack" function={openSlackWithCommand} />
      <SimpleButton text="Notion make child page" function={openNotion}/>
      <SimpleButton text="Open Chrom" function={openChrom} />

      <div>
        <h1>Notion API</h1>
        <button onClick={storeNotionApi}>Open Chanel</button>
      </div>
    </div>
  );
}

export default App;
