import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { useStore } from "./state/store";
import { GroupTree } from "./components/sidebar/GroupTree";
import { ConnectionEditor } from "./components/editor/ConnectionEditor";
import { SettingsPage } from "./components/settings/SettingsPage";
import { DialogHost } from "./components/ui/DialogHost";

export default function App() {
  const { loadAll, loading, searchQuery, setSearchQuery, toast, dismissToast, setSessionActive } =
    useStore();
  const [showSettings, setShowSettings] = useState(false);

  useEffect(() => {
    loadAll();

    let unlistenStarted: (() => void) | undefined;
    let unlistenEnded: (() => void) | undefined;
    (async () => {
      unlistenStarted = await listen<string>("session-started", (e) =>
        setSessionActive(e.payload, true)
      );
      unlistenEnded = await listen<string>("session-ended", (e) =>
        setSessionActive(e.payload, false)
      );
    })();

    return () => {
      unlistenStarted?.();
      unlistenEnded?.();
    };
  }, [loadAll, setSessionActive]);

  if (loading) {
    return (
      <div className="h-screen flex items-center justify-center text-neutral-400 text-sm">
        Loading…
      </div>
    );
  }

  return (
    <div className="h-screen flex flex-col bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100">
      <div className="flex items-center gap-3 px-3 py-2 border-b border-neutral-200 dark:border-neutral-700">
        <h1 className="font-semibold text-sm shrink-0">RDP Manager</h1>
        <input
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          placeholder="Search connections…"
          className="flex-1 max-w-xs px-2.5 py-1 rounded-md border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <div className="flex-1" />
        <button
          onClick={() => setShowSettings((s) => !s)}
          className="text-sm px-2 py-1 rounded hover:bg-neutral-100 dark:hover:bg-neutral-800 text-neutral-600 dark:text-neutral-300"
        >
          ⚙ Settings
        </button>
      </div>

      <div className="flex-1 flex overflow-hidden">
        <aside className="w-64 shrink-0 border-r border-neutral-200 dark:border-neutral-700 overflow-hidden">
          <GroupTree />
        </aside>
        {showSettings ? (
          <SettingsPage onClose={() => setShowSettings(false)} />
        ) : (
          <ConnectionEditor />
        )}
      </div>

      {toast && (
        <div
          className={`fixed bottom-4 right-4 px-4 py-2 rounded-md shadow-lg text-sm text-white cursor-pointer ${
            toast.kind === "error" ? "bg-red-600" : "bg-green-600"
          }`}
          onClick={dismissToast}
        >
          {toast.message}
        </div>
      )}

      <DialogHost />
    </div>
  );
}
