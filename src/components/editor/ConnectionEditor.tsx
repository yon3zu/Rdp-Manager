import { useEffect, useState } from "react";
import { useStore } from "../../state/store";
import { api } from "../../api/tauri";
import {
  defaultAdvancedSettings,
  type ConnectionProfile,
  type ConnectionProfileInput,
} from "../../api/types";
import { Button, Tabs } from "../ui/primitives";
import { GeneralTab } from "./tabs/GeneralTab";
import { DisplayTab } from "./tabs/DisplayTab";
import { LocalResourcesTab } from "./tabs/LocalResourcesTab";
import { GatewayTab } from "./tabs/GatewayTab";
import { AdvancedTab } from "./tabs/AdvancedTab";

const TABS = ["General", "Display", "Local Resources", "Gateway", "Advanced"];

function emptyInput(groupId: string | null): ConnectionProfileInput {
  return {
    group_id: groupId,
    name: "",
    host: "",
    port: 3389,
    username: null,
    domain: null,
    notes: null,
    advanced: defaultAdvancedSettings(),
  };
}

export function ConnectionEditor() {
  const {
    profiles,
    selectedProfileId,
    editingNewProfileGroupId,
    saveProfile,
    cancelEditing,
    launchConnection,
    platform,
    activeSessionIds,
  } = useStore();

  const existing: ConnectionProfile | undefined = profiles.find(
    (p) => p.id === selectedProfileId
  );
  const isNew = editingNewProfileGroupId !== null;

  const [tab, setTab] = useState("General");
  const [input, setInput] = useState<ConnectionProfileInput>(() =>
    existing
      ? {
          group_id: existing.group_id,
          name: existing.name,
          host: existing.host,
          port: existing.port,
          username: existing.username,
          domain: existing.domain,
          notes: existing.notes,
          advanced: existing.advanced,
        }
      : emptyInput(editingNewProfileGroupId === "root" ? null : editingNewProfileGroupId)
  );
  const [password, setPassword] = useState("");
  const [gatewayPassword, setGatewayPassword] = useState("");
  const [hasPassword, setHasPassword] = useState(existing?.has_saved_password ?? false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setTab("General");
    setPassword("");
    setGatewayPassword("");
    if (existing) {
      setInput({
        group_id: existing.group_id,
        name: existing.name,
        host: existing.host,
        port: existing.port,
        username: existing.username,
        domain: existing.domain,
        notes: existing.notes,
        advanced: existing.advanced,
      });
      setHasPassword(existing.has_saved_password);
    } else if (isNew) {
      setInput(emptyInput(editingNewProfileGroupId === "root" ? null : editingNewProfileGroupId));
      setHasPassword(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedProfileId, editingNewProfileGroupId]);

  if (!existing && !isNew) {
    return (
      <div className="flex-1 flex items-center justify-center text-neutral-400 text-sm">
        Pilih koneksi di sidebar, atau buat koneksi baru.
      </div>
    );
  }

  const handleSave = async () => {
    setSaving(true);
    try {
      await saveProfile(
        existing?.id ?? null,
        input,
        password || undefined,
        gatewayPassword || undefined
      );
      setPassword("");
      setGatewayPassword("");
    } finally {
      setSaving(false);
    }
  };

  const handleClearPassword = async () => {
    if (!existing) return;
    await api.clearProfilePassword(existing.id);
    setHasPassword(false);
  };

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 border-b border-neutral-200 dark:border-neutral-700">
        <input
          value={input.name}
          onChange={(e) => setInput({ ...input, name: e.target.value })}
          placeholder="Connection name"
          className="text-lg font-semibold bg-transparent focus:outline-none focus:ring-1 focus:ring-blue-500 rounded px-1 -mx-1 text-neutral-900 dark:text-neutral-100"
        />
        <div className="flex items-center gap-2">
          {existing && activeSessionIds.has(existing.id) && (
            <span className="flex items-center gap-1.5 text-xs text-green-600 dark:text-green-400 px-2">
              <span className="w-1.5 h-1.5 rounded-full bg-green-500" />
              Connected
            </span>
          )}
          {existing && (
            <Button variant="ghost" onClick={() => launchConnection(existing.id)}>
              ▶ Launch
            </Button>
          )}
          <Button variant="secondary" onClick={cancelEditing}>
            Cancel
          </Button>
          <Button onClick={handleSave} disabled={saving}>
            {saving ? "Saving…" : "Save"}
          </Button>
        </div>
      </div>

      <Tabs tabs={TABS} active={tab} onChange={setTab} />

      <div className="flex-1 overflow-y-auto p-4">
        {tab === "General" && (
          <GeneralTab
            input={input}
            setInput={setInput}
            password={password}
            setPassword={setPassword}
            hasPassword={hasPassword}
            onClearPassword={existing ? handleClearPassword : undefined}
          />
        )}
        {tab === "Display" && <DisplayTab input={input} setInput={setInput} />}
        {tab === "Local Resources" && (
          <LocalResourcesTab input={input} setInput={setInput} />
        )}
        {tab === "Gateway" && (
          <GatewayTab
            input={input}
            setInput={setInput}
            gatewayPassword={gatewayPassword}
            setGatewayPassword={setGatewayPassword}
            hasGatewayPassword={existing?.has_saved_gateway_password ?? false}
          />
        )}
        {tab === "Advanced" && (
          <AdvancedTab input={input} setInput={setInput} platform={platform} />
        )}
      </div>
    </div>
  );
}
