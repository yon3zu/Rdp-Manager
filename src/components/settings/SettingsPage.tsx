import { useState } from "react";
import { useStore } from "../../state/store";
import { api } from "../../api/tauri";
import { Button } from "../ui/primitives";

export function SettingsPage({ onClose }: { onClose: () => void }) {
  const { platform, launcherStatus, showToast } = useStore();
  const [copied, setCopied] = useState(false);
  const [thumbprint, setThumbprint] = useState<string | null>(null);
  const [loadingThumbprint, setLoadingThumbprint] = useState(false);
  const [thumbprintCopied, setThumbprintCopied] = useState(false);

  const copyInstallCommand = async () => {
    await navigator.clipboard.writeText("brew install freerdp");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const generateCertificate = async () => {
    setLoadingThumbprint(true);
    try {
      const tp = await api.getSigningThumbprint();
      setThumbprint(tp);
    } catch (e) {
      showToast("error", String(e));
    } finally {
      setLoadingThumbprint(false);
    }
  };

  const copyThumbprint = async () => {
    if (!thumbprint) return;
    await navigator.clipboard.writeText(thumbprint);
    setThumbprintCopied(true);
    setTimeout(() => setThumbprintCopied(false), 2000);
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

        {platform === "windows" && (
          <section className="flex flex-col gap-2">
            <h3 className="text-sm font-semibold text-neutral-700 dark:text-neutral-300">
              RDP file signing
            </h3>
            <p className="text-sm text-neutral-600 dark:text-neutral-400">
              Windows shows a security warning ("unknown publisher") every time you launch an
              unsigned .rdp file. Generate a local certificate and trust it once via Group Policy
              to suppress this warning for every connection launched by this app.
            </p>
            {!thumbprint ? (
              <Button onClick={generateCertificate} disabled={loadingThumbprint} className="self-start">
                {loadingThumbprint ? "Generating…" : "Generate signing certificate"}
              </Button>
            ) : (
              <>
                <div className="flex items-center gap-2">
                  <code className="bg-neutral-100 dark:bg-neutral-800 px-2 py-1 rounded text-xs break-all">
                    {thumbprint}
                  </code>
                  <Button variant="secondary" onClick={copyThumbprint}>
                    {thumbprintCopied ? "Copied!" : "Copy"}
                  </Button>
                </div>
                <ol className="text-sm text-neutral-600 dark:text-neutral-400 list-decimal list-inside flex flex-col gap-1">
                  <li>
                    Open <code className="bg-neutral-100 dark:bg-neutral-800 px-1 rounded">gpedit.msc</code> (Local
                    Group Policy Editor — requires Windows Pro/Enterprise)
                  </li>
                  <li>
                    Go to Computer Configuration → Administrative Templates → Windows Components →
                    Remote Desktop Services → Remote Desktop Connection Client
                  </li>
                  <li>
                    Enable "Specify thumbprints of certificates representing trusted .rdp
                    publishers" and paste the thumbprint above
                  </li>
                </ol>
                <p className="text-xs text-neutral-500">
                  This is a one-time setup per Windows machine — not per connection. New
                  connections launched by this app are signed automatically from now on.
                </p>
              </>
            )}
          </section>
        )}

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
