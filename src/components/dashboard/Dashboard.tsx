import { useEffect, useState } from "react";
import { useStore } from "../../state/store";
import { api } from "../../api/tauri";
import { Button } from "../ui/primitives";

function formatDuration(ms: number): string {
  const totalSeconds = Math.max(0, Math.floor(ms / 1000));
  const h = Math.floor(totalSeconds / 3600);
  const m = Math.floor((totalSeconds % 3600) / 60);
  const s = totalSeconds % 60;
  const pad = (n: number) => n.toString().padStart(2, "0");
  return h > 0 ? `${h}:${pad(m)}:${pad(s)}` : `${m}:${pad(s)}`;
}

export function Dashboard({ onClose }: { onClose: () => void }) {
  const {
    profiles,
    groups,
    activeSessionIds,
    sessionStartedAt,
    disconnectSession,
    selectProfile,
    showToast,
  } = useStore();
  const [, setTick] = useState(0);

  useEffect(() => {
    const id = setInterval(() => setTick((t) => t + 1), 1000);
    return () => clearInterval(id);
  }, []);

  const active = profiles.filter((p) => activeSessionIds.has(p.id));
  const groupName = (id: string | null) => groups.find((g) => g.id === id)?.name ?? null;

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 border-b border-neutral-200 dark:border-neutral-700">
        <h2 className="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
          Dashboard
        </h2>
        <Button variant="secondary" onClick={onClose}>
          Close
        </Button>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        {active.length === 0 ? (
          <p className="text-sm text-neutral-500">
            Tidak ada koneksi yang sedang aktif. Buka salah satu koneksi dan klik Launch.
          </p>
        ) : (
          <div className="grid grid-cols-[repeat(auto-fill,minmax(220px,1fr))] gap-3">
            {active.map((p) => {
              const startedAt = sessionStartedAt[p.id];
              return (
                <div
                  key={p.id}
                  onClick={async () => {
                    try {
                      await api.focusSession(p.id);
                    } catch (e) {
                      showToast("error", String(e));
                    }
                  }}
                  title="Click to jump to this session's window"
                  className="group cursor-pointer rounded-lg border border-neutral-200 dark:border-neutral-700 bg-white dark:bg-neutral-800 p-4 flex flex-col gap-2.5 min-h-[110px] hover:border-blue-400 dark:hover:border-blue-500 hover:shadow-md transition-all"
                >
                  <div className="flex items-center gap-2">
                    <span className="w-2 h-2 rounded-full bg-green-500 shrink-0 animate-pulse" />
                    <span className="font-medium text-sm text-neutral-900 dark:text-neutral-100 truncate flex-1">
                      {p.name}
                    </span>
                    <button
                      title="Edit connection"
                      onClick={(e) => {
                        e.stopPropagation();
                        selectProfile(p.id);
                        onClose();
                      }}
                      className="opacity-0 group-hover:opacity-100 text-neutral-400 hover:text-blue-600 text-xs px-1 shrink-0"
                    >
                      ⚙
                    </button>
                  </div>
                  <div className="text-xs text-neutral-500 truncate" title={`${p.host}:${p.port}`}>
                    {p.host}:{p.port}
                  </div>
                  {groupName(p.group_id) && (
                    <span className="self-start text-[11px] px-1.5 py-0.5 rounded bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300">
                      {groupName(p.group_id)}
                    </span>
                  )}
                  <div className="flex items-center justify-between mt-1">
                    <span className="text-xs font-mono text-neutral-500">
                      {startedAt ? formatDuration(Date.now() - startedAt) : "—"}
                    </span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        disconnectSession(p.id);
                      }}
                      className="text-xs text-red-500 hover:text-red-600 px-2 py-0.5 rounded hover:bg-red-50 dark:hover:bg-red-900/20"
                    >
                      Disconnect
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
