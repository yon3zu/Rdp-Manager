export interface Group {
  id: string;
  name: string;
  parent_id: string | null;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export type ScreenMode = "windowed" | "fullscreen";
export type AudioMode = "local" | "remote" | "none";
export type GatewayUsage = "none" | "always" | "detect";
export type CertTrustBehavior = "prompt" | "trust_on_first_use" | "ignore" | "deny";

export interface AdvancedSettings {
  screen_mode: ScreenMode;
  desktop_width: number | null;
  desktop_height: number | null;
  dynamic_resolution: boolean;
  color_depth: number;
  multi_monitor: boolean;
  selected_monitors: string | null;
  admin_session: boolean;
  redirect_drives: boolean;
  redirect_printers: boolean;
  redirect_clipboard: boolean;
  audio_mode: AudioMode;
  mic_redirection: boolean;
  gateway_hostname: string | null;
  gateway_port: number | null;
  gateway_username: string | null;
  gateway_domain: string | null;
  gateway_usage: GatewayUsage;
  cert_trust_behavior: CertTrustBehavior;
  connection_timeout_ms: number | null;
}

export const defaultAdvancedSettings = (): AdvancedSettings => ({
  screen_mode: "windowed",
  desktop_width: 1280,
  desktop_height: 800,
  dynamic_resolution: true,
  color_depth: 32,
  multi_monitor: false,
  selected_monitors: null,
  admin_session: false,
  redirect_drives: false,
  redirect_printers: false,
  redirect_clipboard: true,
  audio_mode: "local",
  mic_redirection: false,
  gateway_hostname: null,
  gateway_port: null,
  gateway_username: null,
  gateway_domain: null,
  gateway_usage: "none",
  cert_trust_behavior: "prompt",
  connection_timeout_ms: null,
});

export interface ConnectionProfile {
  id: string;
  group_id: string | null;
  name: string;
  protocol: string;
  host: string;
  port: number;
  username: string | null;
  domain: string | null;
  has_saved_password: boolean;
  has_saved_gateway_password: boolean;
  sort_order: number;
  notes: string | null;
  advanced: AdvancedSettings;
  created_at: string;
  updated_at: string;
}

export interface ConnectionProfileInput {
  group_id: string | null;
  name: string;
  host: string;
  port: number;
  username: string | null;
  domain: string | null;
  notes: string | null;
  advanced: AdvancedSettings;
}

export interface LauncherReadiness {
  available: boolean;
  detail: string | null;
}

export type Platform = "windows" | "macos" | "other";
