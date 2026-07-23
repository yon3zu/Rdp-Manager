import { invoke } from "@tauri-apps/api/core";
import type {
  ConnectionProfile,
  ConnectionProfileInput,
  Group,
  LauncherReadiness,
  Platform,
} from "./types";

export const api = {
  listGroups: () => invoke<Group[]>("list_groups"),
  createGroup: (name: string, parentId: string | null) =>
    invoke<Group>("create_group", { name, parentId }),
  renameGroup: (id: string, name: string) => invoke<void>("rename_group", { id, name }),
  moveGroup: (id: string, newParentId: string | null, newOrder: number) =>
    invoke<void>("move_group", { id, newParentId, newOrder }),
  deleteGroup: (id: string) => invoke<void>("delete_group", { id }),

  listProfiles: (groupId: string | null) =>
    invoke<ConnectionProfile[]>("list_profiles", { groupId }),
  getProfile: (id: string) => invoke<ConnectionProfile>("get_profile", { id }),
  createProfile: (input: ConnectionProfileInput) =>
    invoke<ConnectionProfile>("create_profile", { input }),
  updateProfile: (id: string, input: ConnectionProfileInput) =>
    invoke<ConnectionProfile>("update_profile", { id, input }),
  deleteProfile: (id: string) => invoke<void>("delete_profile", { id }),
  duplicateProfile: (id: string) => invoke<ConnectionProfile>("duplicate_profile", { id }),
  moveProfile: (id: string, groupId: string | null, order: number) =>
    invoke<void>("move_profile", { id, groupId, order }),

  setProfilePassword: (profileId: string, password: string) =>
    invoke<void>("set_profile_password", { profileId, password }),
  clearProfilePassword: (profileId: string) =>
    invoke<void>("clear_profile_password", { profileId }),
  hasProfilePassword: (profileId: string) =>
    invoke<boolean>("has_profile_password", { profileId }),
  setGatewayPassword: (profileId: string, password: string) =>
    invoke<void>("set_gateway_password", { profileId, password }),
  clearGatewayPassword: (profileId: string) =>
    invoke<void>("clear_gateway_password", { profileId }),

  launchConnection: (profileId: string) => invoke<void>("launch_connection", { profileId }),
  exportRdpFile: (profileId: string, destPath: string) =>
    invoke<void>("export_rdp_file", { profileId, destPath }),
  listActiveSessions: () => invoke<string[]>("list_active_sessions"),

  checkLauncherStatus: () => invoke<LauncherReadiness>("check_launcher_status"),
  getPlatform: () => invoke<Platform>("get_platform"),
  getSigningThumbprint: () => invoke<string>("get_signing_thumbprint"),
};
