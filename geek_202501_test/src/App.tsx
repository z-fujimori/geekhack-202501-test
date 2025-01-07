// import { invoke } from '@tauri-apps/api/tauri';

import { invoke } from "@tauri-apps/api/core";

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
      await invoke("store_notion_api");
    } catch (err) {
      console.error("failed store_notion_api", err);
    }
  }

  return (
    <div>
      <div>
        <h1>Open Slack Desktop App</h1>
        <button onClick={openSlackApp}>Open Slack</button>
      </div>
      <div>
        <h1>Open Slack Chanel</h1>
        <button onClick={openSlackWithCommand}>Open Chanel</button>
      </div>
      <div>
        <h1>Open Notion</h1>
        <button onClick={openNotion}>Open Chanel</button>
      </div>
      <div>
        <h1>Open Chrome</h1>
        <button onClick={openChrom}>Open Chanel</button>
      </div>
      <div>
        <h1>Notion API</h1>
        <button onClick={storeNotionApi}>Open Chanel</button>
      </div>
    </div>
  );
}

export default App;
