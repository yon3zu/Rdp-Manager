import type { ConnectionProfileInput } from "../../../api/types";
import { Field, Select, TextInput, Toggle } from "../../ui/primitives";

export function DisplayTab({
  input,
  setInput,
}: {
  input: ConnectionProfileInput;
  setInput: (i: ConnectionProfileInput) => void;
}) {
  const a = input.advanced;
  const set = (patch: Partial<typeof a>) => setInput({ ...input, advanced: { ...a, ...patch } });

  return (
    <div className="grid grid-cols-2 gap-4 max-w-xl">
      <Field label="Screen mode">
        <Select
          value={a.screen_mode}
          onChange={(e) => set({ screen_mode: e.target.value as "windowed" | "fullscreen" })}
        >
          <option value="windowed">Windowed</option>
          <option value="fullscreen">Fullscreen</option>
        </Select>
      </Field>
      <Field label="Color depth">
        <Select value={a.color_depth} onChange={(e) => set({ color_depth: Number(e.target.value) })}>
          <option value={16}>16-bit</option>
          <option value={24}>24-bit</option>
          <option value={32}>32-bit (True Color)</option>
        </Select>
      </Field>

      {a.screen_mode === "windowed" && !a.dynamic_resolution && (
        <>
          <Field label="Width">
            <TextInput
              type="number"
              value={a.desktop_width ?? 1280}
              onChange={(e) => set({ desktop_width: Number(e.target.value) })}
            />
          </Field>
          <Field label="Height">
            <TextInput
              type="number"
              value={a.desktop_height ?? 800}
              onChange={(e) => set({ desktop_height: Number(e.target.value) })}
            />
          </Field>
        </>
      )}

      <div className="col-span-2 flex flex-col gap-2 pt-2">
        <Toggle
          checked={a.dynamic_resolution}
          onChange={(v) => set({ dynamic_resolution: v })}
          label="Dynamic resolution (resize with window)"
        />
        <Toggle
          checked={a.multi_monitor}
          onChange={(v) => set({ multi_monitor: v })}
          label="Use all monitors"
        />
        {a.multi_monitor && (
          <Field label="Monitor IDs" hint="Comma-separated, leave blank for all">
            <TextInput
              value={a.selected_monitors ?? ""}
              onChange={(e) => set({ selected_monitors: e.target.value || null })}
              placeholder="0,1"
            />
          </Field>
        )}
        <Toggle
          checked={a.admin_session}
          onChange={(v) => set({ admin_session: v })}
          label="Connect to admin/console session"
        />
      </div>
    </div>
  );
}
