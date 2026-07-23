import { useState } from "react";
import { useStore } from "../../state/store";
import { Button } from "../ui/primitives";

export function SettingsPage({ onClose }: { onClose: () => void }) {
  const { platform, launcherStatus } = useStore();
  const [copied, setCopied] = useState(false);

  const copyInstallCommand = async () => {
    await navigator.clipboard.writeText("brew install freerdp");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 border-b border-neutral-200 dark:border-neutral-700">
        <h2 className="text-lg font-semibold text-neutral-900 dark:text-neutral-100">Settings</h2>
        <Button variant="secondary" onClick={onClose}>
          Close
        </Button>
      </div>
      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-6 max-w-xl">
        <section className="flex flex-col gap-2">
          <h3 className="text-sm font-semibold text-neutral-700 dark:text-neutral-300">
            RDP Launcher
          </h3>
          <p className="text-sm text-neutral-600 dark:text-neutral-400">
            Platform: <span className="font-mono">{platform ?? "detecting…"}</span>
          </p>
          {launcherStatus && (
            <div
              className={`text-sm rounded-md px-3 py-2 ${
                launcherStatus.available
                  ? "bg-green-50 text-green-800 dark:bg-green-900/30 dark:text-green-300"
                  : "bg-amber-50 text-amber-800 dark:bg-amber-900/30 dark:text-amber-300"
              }`}
            >
              {launcherStatus.available ? "✓ Ready: " : "⚠ Not found: "}
              {launcherStatus.detail}
            </div>
          )}
          {platform === "macos" && !launcherStatus?.available && (
            <div className="flex items-center gap-2">
              <code className="bg-neutral-100 dark:bg-neutral-800 px-2 py-1 rounded text-sm">
                brew install freerdp
              </code>
              <Button variant="secondary" onClick={copyInstallCommand}>
                {copied ? "Copied!" : "Copy"}
              </Button>
            </div>
          )}
          {platform === "windows" && (
            <p className="text-sm text-neutral-500">
              mstsc.exe is built into Windows — no install needed.
            </p>
          )}
        </section>

        <section className="flex flex-col gap-2">
          <h3 className="text-sm font-semibold text-neutral-700 dark:text-neutral-300">About</h3>
          <p className="text-sm text-neutral-600 dark:text-neutral-400">
            RDP Manager — lightweight remote desktop connection manager. Connections launch your
            OS's native RDP client; clipboard redirection is handled by that client, not by this
            app.
          </p>
        </section>
      </div>
    </div>
  );
}
