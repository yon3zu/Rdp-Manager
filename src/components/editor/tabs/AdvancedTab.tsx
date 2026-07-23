import type { ConnectionProfileInput, Platform } from "../../../api/types";
import { Field, Select, TextInput } from "../../ui/primitives";

export function AdvancedTab({
  input,
  setInput,
  platform,
}: {
  input: ConnectionProfileInput;
  setInput: (i: ConnectionProfileInput) => void;
  platform: Platform | null;
}) {
  const a = input.advanced;
  const set = (patch: Partial<typeof a>) => setInput({ ...input, advanced: { ...a, ...patch } });
  const isWindows = platform === "windows";

  return (
    <div className="grid grid-cols-2 gap-4 max-w-xl">
      <div className="col-span-2">
        <Field
          label="Certificate trust"
          hint={
            isWindows
              ? "mstsc/Windows only partially honors this — the OS trust store and Group Policy may still prompt."
              : "Passed to xfreerdp as /cert:deny, /cert:ignore, or /cert:tofu."
          }
        >
          <Select
            value={a.cert_trust_behavior}
            onChange={(e) => set({ cert_trust_behavior: e.target.value as typeof a.cert_trust_behavior })}
            className="max-w-xs"
          >
            <option value="prompt">Prompt each time</option>
            <option value="trust_on_first_use">Trust on first use</option>
            <option value="ignore">Ignore certificate errors</option>
            <option value="deny">Deny on certificate errors</option>
          </Select>
        </Field>
      </div>

      <div className="col-span-2">
        <Field
          label="Connection timeout (ms)"
          hint={isWindows ? "macOS only — mstsc has no equivalent .rdp setting." : "Passed to xfreerdp as /timeout:<ms>"}
        >
          <TextInput
            type="number"
            disabled={isWindows}
            value={a.connection_timeout_ms ?? ""}
            onChange={(e) =>
              set({ connection_timeout_ms: e.target.value ? Number(e.target.value) : null })
            }
            className="max-w-xs"
          />
        </Field>
      </div>
    </div>
  );
}
