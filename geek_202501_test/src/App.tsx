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
    </div>
  );
}

export default App;
