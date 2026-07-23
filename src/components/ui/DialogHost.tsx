import { useEffect, useState } from "react";
import { useStore } from "../../state/store";
import { Button, TextInput } from "./primitives";

export function DialogHost() {
  const { dialog, resolveDialog } = useStore();
  const [value, setValue] = useState("");

  useEffect(() => {
    if (dialog?.kind === "prompt") setValue(dialog.defaultValue);
  }, [dialog]);

  if (!dialog) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="w-80 rounded-lg bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 shadow-xl p-4 flex flex-col gap-3">
        <p className="text-sm text-neutral-800 dark:text-neutral-100">{dialog.message}</p>
        {dialog.kind === "prompt" && (
          <TextInput
            autoFocus
            value={value}
            onChange={(e) => setValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") resolveDialog(value);
              if (e.key === "Escape") resolveDialog(null);
            }}
          />
        )}
        <div className="flex justify-end gap-2 pt-1">
          <Button variant="secondary" onClick={() => resolveDialog(dialog.kind === "prompt" ? null : false)}>
            Cancel
          </Button>
          <Button onClick={() => resolveDialog(dialog.kind === "prompt" ? value : true)}>OK</Button>
        </div>
      </div>
    </div>
  );
}
