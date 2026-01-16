import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [view, setView] = useState<"home" | "settings">("home");

  useEffect(() => {
    const unlisten = listen<string>("navigate", (event) => {
      console.log("Navigating to", event.payload);
      if (event.payload === "settings") {
        setView("settings");
      } else {
        setView("home");
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <main className="container">
      {view === "home" && (
        <div className="view-home">
          <h1>Malax Dashboard</h1>
          <p>Status: Running</p>
          <p>Model: mlx-model (Loaded)</p>
          <div className="actions">
            <button onClick={() => setView("settings")}>Go to Settings</button>
          </div>
        </div>
      )}

      {view === "settings" && (
        <div className="view-settings">
          <h1>Settings</h1>
          <form className="settings-form">
            <div className="form-group">
              <label>Model Path</label>
              <input type="text" placeholder="/path/to/models" />
            </div>
            <div className="form-group">
              <label>Port</label>
              <input type="number" defaultValue={8080} />
            </div>
            <div className="form-group">
              <label>API Key</label>
              <input type="password" placeholder="sk-..." />
            </div>
          </form>
          <button onClick={() => setView("home")}>Back to Dashboard</button>
        </div>
      )}
    </main>
  );
}

export default App;
