import { create } from "zustand";
import { api } from "../api/tauri";
import type {
  ConnectionProfile,
  ConnectionProfileInput,
  Group,
  LauncherReadiness,
  Platform,
} from "../api/types";

interface AppState {
  groups: Group[];
  profiles: ConnectionProfile[];
  selectedGroupId: string | null;
  selectedProfileId: string | null;
  editingNewProfileGroupId: string | null;
  searchQuery: string;
  platform: Platform | null;
  launcherStatus: LauncherReadiness | null;
  activeSessionIds: Set<string>;
  loading: boolean;
  toast: { kind: "error" | "success"; message: string } | null;

  loadAll: () => Promise<void>;
  selectGroup: (id: string | null) => void;
  selectProfile: (id: string | null) => void;
  startNewProfile: (groupId: string | null) => void;
  cancelEditing: () => void;
  setSearchQuery: (q: string) => void;

  createGroup: (name: string, parentId: string | null) => Promise<void>;
  renameGroup: (id: string, name: string) => Promise<void>;
  deleteGroup: (id: string) => Promise<void>;

  saveProfile: (
    id: string | null,
    input: ConnectionProfileInput,
    password?: string,
    gatewayPassword?: string
  ) => Promise<void>;
  deleteProfile: (id: string) => Promise<void>;
  duplicateProfile: (id: string) => Promise<void>;
  launchConnection: (id: string) => Promise<void>;
  setSessionActive: (profileId: string, active: boolean) => void;

  showToast: (kind: "error" | "success", message: string) => void;
  dismissToast: () => void;
}

export const useStore = create<AppState>((set, get) => ({
  groups: [],
  profiles: [],
  selectedGroupId: null,
  selectedProfileId: null,
  editingNewProfileGroupId: null,
  searchQuery: "",
  platform: null,
  launcherStatus: null,
  activeSessionIds: new Set(),
  loading: true,
  toast: null,

  loadAll: async () => {
    set({ loading: true });
    try {
      const [groups, profiles, platform, launcherStatus, activeSessions] = await Promise.all([
        api.listGroups(),
        api.listProfiles(null),
        api.getPlatform(),
        api.checkLauncherStatus(),
        api.listActiveSessions(),
      ]);
      set({
        groups,
        profiles,
        platform,
        launcherStatus,
        activeSessionIds: new Set(activeSessions),
        loading: false,
      });
    } catch (e) {
      set({ loading: false });
      get().showToast("error", String(e));
    }
  },

  selectGroup: (id) => set({ selectedGroupId: id }),
  selectProfile: (id) =>
    set({ selectedProfileId: id, editingNewProfileGroupId: null }),
  startNewProfile: (groupId) =>
    set({ editingNewProfileGroupId: groupId ?? "root", selectedProfileId: null }),
  cancelEditing: () => set({ editingNewProfileGroupId: null }),
  setSearchQuery: (q) => set({ searchQuery: q }),

  createGroup: async (name, parentId) => {
    try {
      const group = await api.createGroup(name, parentId);
      set({ groups: [...get().groups, group] });
    } catch (e) {
      get().showToast("error", String(e));
    }
  },
  renameGroup: async (id, name) => {
    try {
      await api.renameGroup(id, name);
      set({
        groups: get().groups.map((g) => (g.id === id ? { ...g, name } : g)),
      });
    } catch (e) {
      get().showToast("error", String(e));
    }
  },
  deleteGroup: async (id) => {
    try {
      await api.deleteGroup(id);
      set({
        groups: get().groups.filter((g) => g.id !== id),
        profiles: get().profiles.map((p) =>
          p.group_id === id ? { ...p, group_id: null } : p
        ),
        selectedGroupId: get().selectedGroupId === id ? null : get().selectedGroupId,
      });
    } catch (e) {
      get().showToast("error", String(e));
    }
  },

  saveProfile: async (id, input, password, gatewayPassword) => {
    try {
      const saved = id
        ? await api.updateProfile(id, input)
        : await api.createProfile(input);

      if (password) {
        await api.setProfilePassword(saved.id, password);
        saved.has_saved_password = true;
      }
      if (gatewayPassword) {
        await api.setGatewayPassword(saved.id, gatewayPassword);
        saved.has_saved_gateway_password = true;
      }

      const existing = get().profiles;
      const next = id
        ? existing.map((p) => (p.id === id ? saved : p))
        : [...existing, saved];
      set({
        profiles: next,
        selectedProfileId: saved.id,
        editingNewProfileGroupId: null,
      });
      get().showToast("success", `Saved "${saved.name}"`);
    } catch (e) {
      get().showToast("error", String(e));
      throw e;
    }
  },
  deleteProfile: async (id) => {
    try {
      await api.deleteProfile(id);
      set({
        profiles: get().profiles.filter((p) => p.id !== id),
        selectedProfileId: get().selectedProfileId === id ? null : get().selectedProfileId,
      });
    } catch (e) {
      get().showToast("error", String(e));
    }
  },
  duplicateProfile: async (id) => {
    try {
      const copy = await api.duplicateProfile(id);
      set({ profiles: [...get().profiles, copy], selectedProfileId: copy.id });
    } catch (e) {
      get().showToast("error", String(e));
    }
  },
  launchConnection: async (id) => {
    try {
      await api.launchConnection(id);
    } catch (e) {
      get().showToast("error", String(e));
    }
  },
  setSessionActive: (profileId, active) => {
    const next = new Set(get().activeSessionIds);
    if (active) next.add(profileId);
    else next.delete(profileId);
    set({ activeSessionIds: next });
  },

  showToast: (kind, message) => {
    set({ toast: { kind, message } });
    setTimeout(() => {
      if (get().toast?.message === message) set({ toast: null });
    }, 5000);
  },
  dismissToast: () => set({ toast: null }),
}));
