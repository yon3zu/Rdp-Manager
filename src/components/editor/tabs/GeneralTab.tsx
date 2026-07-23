import { useMemo } from "react";
import type { ConnectionProfileInput, Group } from "../../../api/types";
import { useStore } from "../../../state/store";
import { Button, Field, Select, TextInput } from "../../ui/primitives";

function flattenGroups(groups: Group[]): { id: string; label: string }[] {
  const byParent = new Map<string | null, Group[]>();
  for (const g of groups) {
    const key = g.parent_id;
    if (!byParent.has(key)) byParent.set(key, []);
    byParent.get(key)!.push(g);
  }
  const out: { id: string; label: string }[] = [];
  function walk(parentId: string | null, depth: number) {
    for (const g of byParent.get(parentId) ?? []) {
      out.push({ id: g.id, label: `${"— ".repeat(depth)}${g.name}` });
      walk(g.id, depth + 1);
    }
  }
  walk(null, 0);
  return out;
}

export function GeneralTab({
  input,
  setInput,
  password,
  setPassword,
  hasPassword,
  onClearPassword,
}: {
  input: ConnectionProfileInput;
  setInput: (i: ConnectionProfileInput) => void;
  password: string;
  setPassword: (v: string) => void;
  hasPassword: boolean;
  onClearPassword?: () => void;
}) {
  const groups = useStore((s) => s.groups);
  const groupOptions = useMemo(() => flattenGroups(groups), [groups]);

  return (
    <div className="grid grid-cols-2 gap-4 max-w-xl">
      <div className="col-span-2">
        <Field label="Group">
          <Select
            value={input.group_id ?? ""}
            onChange={(e) => setInput({ ...input, group_id: e.target.value || null })}
            className="max-w-xs"
          >
            <option value="">No group</option>
            {groupOptions.map((g) => (
              <option key={g.id} value={g.id}>
                {g.label}
              </option>
            ))}
          </Select>
        </Field>
      </div>
      <Field label="Host">
        <TextInput
          value={input.host}
          onChange={(e) => setInput({ ...input, host: e.target.value })}
          placeholder="192.168.1.10 or hostname"
        />
      </Field>
      <Field label="Port">
        <TextInput
          type="number"
          value={input.port}
          onChange={(e) => setInput({ ...input, port: Number(e.target.value) || 3389 })}
        />
      </Field>
      <Field label="Username">
        <TextInput
          value={input.username ?? ""}
          onChange={(e) => setInput({ ...input, username: e.target.value || null })}
        />
      </Field>
      <Field label="Domain">
        <TextInput
          value={input.domain ?? ""}
          onChange={(e) => setInput({ ...input, domain: e.target.value || null })}
          placeholder="optional"
        />
      </Field>
      <div className="col-span-2">
        <Field
          label="Password"
          hint="Disimpan aman di OS keychain, tidak pernah di database lokal."
        >
          <div className="flex gap-2 items-center">
            <TextInput
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder={hasPassword ? "•••••••• (saved — leave blank to keep)" : "Enter password"}
              className="flex-1"
            />
            {hasPassword && onClearPassword && (
              <Button variant="ghost" onClick={onClearPassword}>
                Clear
              </Button>
            )}
          </div>
        </Field>
      </div>
      <div className="col-span-2">
        <Field label="Notes">
          <textarea
            value={input.notes ?? ""}
            onChange={(e) => setInput({ ...input, notes: e.target.value || null })}
            rows={3}
            className="px-2.5 py-1.5 rounded-md border border-neutral-300 dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </Field>
      </div>
    </div>
  );
}
