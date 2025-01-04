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

  return (
    <div>
      <h1>Open Slack Desktop App</h1>
      <button onClick={openSlackApp}>Open Slack</button>
    </div>
  );
}

export default App;
