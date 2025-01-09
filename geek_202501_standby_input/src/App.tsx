import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [progress, setProgress] = useState<string>("");

  useEffect(() => {
    // バックエンドからの進行状況を受け取る
    const unlisten = listen("pro", (event) => {
      setProgress(event.payload as string);
    });
    // アンマウント時にリスナーを解除
    return () => {
      unlisten.then((unsub) => unsub());
    };
  }, []);

  const startProcessing = async () => {
    try {
      await invoke("start_processing");
    } catch (error) {
      console.error("Error starting processing:", error);
    }
  };
  const pauseProcessing = async () => {
    try {
      await invoke("pause_processing");
    } catch (error) {
      console.error("Error pausing processing:", error);
    }
  };
  const resumeProcessing = async () => {
    try {
      await invoke("resume_processing");
    } catch (error) {
      console.error("Error resuming processing:", error);
    }
  };

  const startInputProcessing = async () => {
    try {
      await invoke("input_two_word");
    } catch (e) {
      console.error(e);
    }
  }
  const resumeInputProcessing = async () => {
    try {
      await invoke("resume_input");
    } catch (e) {
      console.error(e);
    }
  }

  return (
    <div>
      <h1>Tauri: Pause and Resume Processing</h1>
      <button onClick={startProcessing}>Start Processing</button>
      <button onClick={pauseProcessing}>Pause Processing</button>
      <button onClick={resumeProcessing}>Resume Processing</button>
      <div>
        <h2>Progress:</h2>
        <p>{progress}</p>
      </div>
      <div>
        <h3>文字途中入力</h3>
        <div>
          <button onClick={startInputProcessing}>Start input</button>
          <button onClick={resumeInputProcessing}>Second start input</button>
        </div>
      </div>
    </div>
  );
}

export default App;