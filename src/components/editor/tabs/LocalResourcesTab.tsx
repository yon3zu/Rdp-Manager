import type { ConnectionProfileInput } from "../../../api/types";
import { Field, Select, Toggle } from "../../ui/primitives";

export function LocalResourcesTab({
  input,
  setInput,
}: {
  input: ConnectionProfileInput;
  setInput: (i: ConnectionProfileInput) => void;
}) {
  const a = input.advanced;
  const set = (patch: Partial<typeof a>) => setInput({ ...input, advanced: { ...a, ...patch } });

  return (
    <div className="flex flex-col gap-3 max-w-xl">
      <Toggle
        checked={a.redirect_clipboard}
        onChange={(v) => set({ redirect_clipboard: v })}
        label="Redirect clipboard (copy/paste between local PC and remote session)"
      />
      <Toggle
        checked={a.redirect_drives}
        onChange={(v) => set({ redirect_drives: v })}
        label="Redirect local drives"
      />
      <Toggle
        checked={a.redirect_printers}
        onChange={(v) => set({ redirect_printers: v })}
        label="Redirect local printers"
      />
      <Toggle
        checked={a.mic_redirection}
        onChange={(v) => set({ mic_redirection: v })}
        label="Redirect microphone"
      />
      <Field label="Audio playback">
        <Select
          value={a.audio_mode}
          onChange={(e) => set({ audio_mode: e.target.value as typeof a.audio_mode })}
          className="max-w-xs"
        >
          <option value="local">Play on this computer</option>
          <option value="remote">Play on remote computer</option>
          <option value="none">Do not play</option>
        </Select>
      </Field>
    </div>
  );
}
