import type { ConnectionProfileInput } from "../../../api/types";
import { Field, Select, TextInput } from "../../ui/primitives";

export function GatewayTab({
  input,
  setInput,
  gatewayPassword,
  setGatewayPassword,
  hasGatewayPassword,
}: {
  input: ConnectionProfileInput;
  setInput: (i: ConnectionProfileInput) => void;
  gatewayPassword: string;
  setGatewayPassword: (v: string) => void;
  hasGatewayPassword: boolean;
}) {
  const a = input.advanced;
  const set = (patch: Partial<typeof a>) => setInput({ ...input, advanced: { ...a, ...patch } });
  const enabled = a.gateway_usage !== "none";

  return (
    <div className="grid grid-cols-2 gap-4 max-w-xl">
      <Field label="Gateway usage">
        <Select
          value={a.gateway_usage}
          onChange={(e) => set({ gateway_usage: e.target.value as typeof a.gateway_usage })}
        >
          <option value="none">Don't use a gateway</option>
          <option value="always">Always use gateway</option>
          <option value="detect">Detect automatically</option>
        </Select>
      </Field>
      <div />

      {enabled && (
        <>
          <Field label="Gateway hostname">
            <TextInput
              value={a.gateway_hostname ?? ""}
              onChange={(e) => set({ gateway_hostname: e.target.value || null })}
              placeholder="gateway.example.com"
            />
          </Field>
          <Field label="Gateway port" hint="optional, default 443">
            <TextInput
              type="number"
              value={a.gateway_port ?? ""}
              onChange={(e) =>
                set({ gateway_port: e.target.value ? Number(e.target.value) : null })
              }
            />
          </Field>
          <Field label="Gateway username">
            <TextInput
              value={a.gateway_username ?? ""}
              onChange={(e) => set({ gateway_username: e.target.value || null })}
            />
          </Field>
          <Field label="Gateway domain">
            <TextInput
              value={a.gateway_domain ?? ""}
              onChange={(e) => set({ gateway_domain: e.target.value || null })}
            />
          </Field>
          <div className="col-span-2">
            <Field
              label="Gateway password"
              hint="Stored separately in the OS keychain from the session password."
            >
              <TextInput
                type="password"
                value={gatewayPassword}
                onChange={(e) => setGatewayPassword(e.target.value)}
                placeholder={
                  hasGatewayPassword ? "•••••••• (saved — leave blank to keep)" : "Enter password"
                }
              />
            </Field>
          </div>
        </>
      )}
    </div>
  );
}
