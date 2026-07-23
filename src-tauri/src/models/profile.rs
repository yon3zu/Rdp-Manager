use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenMode {
    Windowed,
    Fullscreen,
}

impl ScreenMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScreenMode::Windowed => "windowed",
            ScreenMode::Fullscreen => "fullscreen",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "fullscreen" => ScreenMode::Fullscreen,
            _ => ScreenMode::Windowed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioMode {
    Local,
    Remote,
    None,
}

impl AudioMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioMode::Local => "local",
            AudioMode::Remote => "remote",
            AudioMode::None => "none",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "remote" => AudioMode::Remote,
            "none" => AudioMode::None,
            _ => AudioMode::Local,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GatewayUsage {
    None,
    Always,
    Detect,
}

impl GatewayUsage {
    pub fn as_str(&self) -> &'static str {
        match self {
            GatewayUsage::None => "none",
            GatewayUsage::Always => "always",
            GatewayUsage::Detect => "detect",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "always" => GatewayUsage::Always,
            "detect" => GatewayUsage::Detect,
            _ => GatewayUsage::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertTrustBehavior {
    Prompt,
    TrustOnFirstUse,
    Ignore,
    Deny,
}

impl CertTrustBehavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            CertTrustBehavior::Prompt => "prompt",
            CertTrustBehavior::TrustOnFirstUse => "trust_on_first_use",
            CertTrustBehavior::Ignore => "ignore",
            CertTrustBehavior::Deny => "deny",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "trust_on_first_use" => CertTrustBehavior::TrustOnFirstUse,
            "ignore" => CertTrustBehavior::Ignore,
            "deny" => CertTrustBehavior::Deny,
            _ => CertTrustBehavior::Prompt,
        }
    }
}

/// Advanced RDP settings, mirroring what mstsc/.rdp files and xfreerdp both support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub screen_mode: ScreenMode,
    pub desktop_width: Option<i64>,
    pub desktop_height: Option<i64>,
    pub dynamic_resolution: bool,
    pub color_depth: i64,
    pub multi_monitor: bool,
    pub selected_monitors: Option<String>,
    pub admin_session: bool,
    pub redirect_drives: bool,
    pub redirect_printers: bool,
    pub redirect_clipboard: bool,
    pub audio_mode: AudioMode,
    pub mic_redirection: bool,
    pub gateway_hostname: Option<String>,
    pub gateway_port: Option<i64>,
    pub gateway_username: Option<String>,
    pub gateway_domain: Option<String>,
    pub gateway_usage: GatewayUsage,
    pub cert_trust_behavior: CertTrustBehavior,
    pub connection_timeout_ms: Option<i64>,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            screen_mode: ScreenMode::Windowed,
            desktop_width: Some(1280),
            desktop_height: Some(800),
            dynamic_resolution: true,
            color_depth: 32,
            multi_monitor: false,
            selected_monitors: None,
            admin_session: false,
            redirect_drives: false,
            redirect_printers: false,
            redirect_clipboard: true,
            audio_mode: AudioMode::Local,
            mic_redirection: false,
            gateway_hostname: None,
            gateway_port: None,
            gateway_username: None,
            gateway_domain: None,
            gateway_usage: GatewayUsage::None,
            cert_trust_behavior: CertTrustBehavior::Prompt,
            connection_timeout_ms: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub id: String,
    pub group_id: Option<String>,
    pub name: String,
    pub protocol: String,
    pub host: String,
    pub port: i64,
    pub username: Option<String>,
    pub domain: Option<String>,
    pub has_saved_password: bool,
    pub has_saved_gateway_password: bool,
    pub sort_order: i64,
    pub notes: Option<String>,
    pub advanced: AdvancedSettings,
    pub created_at: String,
    pub updated_at: String,
}

/// Payload sent from the frontend for create/update. Never carries secrets.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionProfileInput {
    pub group_id: Option<String>,
    pub name: String,
    pub host: String,
    pub port: i64,
    pub username: Option<String>,
    pub domain: Option<String>,
    pub notes: Option<String>,
    pub advanced: AdvancedSettings,
}
