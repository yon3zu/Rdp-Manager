pub mod keys;

use crate::models::profile::{CertTrustBehavior, GatewayUsage, ScreenMode};
use crate::models::ConnectionProfile;

/// Builds the `.rdp` file content for a profile. Password is intentionally
/// not embedded (mstsc has no supported way to consume an inline password);
/// the caller is responsible for prompting mstsc's own credential UI or
/// pre-seeding Windows Credential Manager separately.
pub fn generate(profile: &ConnectionProfile) -> String {
    let a = &profile.advanced;
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!(
        "{}:s:{}:{}",
        keys::FULL_ADDRESS,
        profile.host,
        profile.port
    ));

    if let Some(username) = &profile.username {
        let value = match &profile.domain {
            Some(domain) if !domain.trim().is_empty() => format!("{domain}\\{username}"),
            _ => username.clone(),
        };
        lines.push(format!("{}:s:{}", keys::USERNAME, value));
    }

    let screen_mode_id = match a.screen_mode {
        ScreenMode::Windowed => 1,
        ScreenMode::Fullscreen => 2,
    };
    lines.push(format!("{}:i:{}", keys::SCREEN_MODE_ID, screen_mode_id));

    if let Some(w) = a.desktop_width {
        lines.push(format!("{}:i:{}", keys::DESKTOP_WIDTH, w));
    }
    if let Some(h) = a.desktop_height {
        lines.push(format!("{}:i:{}", keys::DESKTOP_HEIGHT, h));
    }
    lines.push(format!(
        "{}:i:{}",
        keys::DYNAMIC_RESOLUTION,
        a.dynamic_resolution as i32
    ));
    lines.push(format!("{}:i:{}", keys::SESSION_BPP, a.color_depth));
    lines.push(format!(
        "{}:i:{}",
        keys::USE_MULTIMON,
        a.multi_monitor as i32
    ));
    if a.multi_monitor {
        if let Some(monitors) = &a.selected_monitors {
            if !monitors.trim().is_empty() {
                lines.push(format!("{}:s:{}", keys::SELECTED_MONITORS, monitors));
            }
        }
    }

    lines.push(format!(
        "{}:i:{}",
        keys::ADMINISTRATIVE_SESSION,
        a.admin_session as i32
    ));
    lines.push(format!(
        "{}:i:{}",
        keys::CONNECT_TO_CONSOLE,
        a.admin_session as i32
    ));

    lines.push(format!(
        "{}:i:{}",
        keys::REDIRECT_DRIVES,
        a.redirect_drives as i32
    ));
    lines.push(format!(
        "{}:i:{}",
        keys::REDIRECT_PRINTERS,
        a.redirect_printers as i32
    ));
    lines.push(format!(
        "{}:i:{}",
        keys::REDIRECT_CLIPBOARD,
        a.redirect_clipboard as i32
    ));

    let audio_mode_id = match a.audio_mode {
        crate::models::profile::AudioMode::Local => 0,
        crate::models::profile::AudioMode::Remote => 1,
        crate::models::profile::AudioMode::None => 2,
    };
    lines.push(format!("{}:i:{}", keys::AUDIO_MODE, audio_mode_id));
    lines.push(format!(
        "{}:i:{}",
        keys::AUDIO_CAPTURE_MODE,
        a.mic_redirection as i32
    ));

    if a.gateway_usage != GatewayUsage::None {
        if let Some(hostname) = &a.gateway_hostname {
            if !hostname.trim().is_empty() {
                lines.push(format!("{}:s:{}", keys::GATEWAY_HOSTNAME, hostname));
            }
        }
        let usage_method = match a.gateway_usage {
            GatewayUsage::None => 0,
            GatewayUsage::Always => 1,
            GatewayUsage::Detect => 2,
        };
        lines.push(format!(
            "{}:i:{}",
            keys::GATEWAY_USAGE_METHOD,
            usage_method
        ));
        lines.push(format!("{}:i:1", keys::GATEWAY_PROFILE_USAGE_METHOD));
    }

    let auth_level = match a.cert_trust_behavior {
        CertTrustBehavior::Prompt => 2,
        CertTrustBehavior::TrustOnFirstUse => 2,
        CertTrustBehavior::Ignore => 0,
        CertTrustBehavior::Deny => 3,
    };
    lines.push(format!("{}:i:{}", keys::AUTHENTICATION_LEVEL, auth_level));

    // Sane defaults, always applied.
    lines.push(format!("{}:i:0", keys::PROMPT_FOR_CREDENTIALS));
    lines.push(format!("{}:i:1", keys::PROMPT_CREDENTIAL_ONCE));
    lines.push(format!("{}:i:1", keys::ENABLE_CREDSSP_SUPPORT));
    lines.push(format!("{}:i:1", keys::NEGOTIATE_SECURITY_LAYER));
    lines.push(format!("{}:i:1", keys::NETWORK_AUTODETECT));
    lines.push(format!("{}:i:1", keys::BANDWIDTH_AUTODETECT));
    lines.push(format!("{}:i:7", keys::CONNECTION_TYPE));

    lines.push(String::new());
    lines.join("\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::profile::{AdvancedSettings, AudioMode};

    fn base_profile() -> ConnectionProfile {
        ConnectionProfile {
            id: "test-id".into(),
            group_id: None,
            name: "Test Server".into(),
            protocol: "rdp".into(),
            host: "192.168.1.10".into(),
            port: 3389,
            username: Some("alice".into()),
            domain: None,
            has_saved_password: false,
            has_saved_gateway_password: false,
            sort_order: 0,
            notes: None,
            advanced: AdvancedSettings::default(),
            created_at: "now".into(),
            updated_at: "now".into(),
        }
    }

    #[test]
    fn generates_full_address_and_username_without_domain() {
        let profile = base_profile();
        let out = generate(&profile);
        assert!(out.contains("full address:s:192.168.1.10:3389"));
        assert!(out.contains("username:s:alice"));
        assert!(!out.contains("username:s:\\alice"));
    }

    #[test]
    fn combines_domain_and_username() {
        let mut profile = base_profile();
        profile.domain = Some("CORP".into());
        let out = generate(&profile);
        assert!(out.contains("username:s:CORP\\alice"));
    }

    #[test]
    fn fullscreen_sets_screen_mode_id_2() {
        let mut profile = base_profile();
        profile.advanced.screen_mode = ScreenMode::Fullscreen;
        let out = generate(&profile);
        assert!(out.contains("screen mode id:i:2"));
    }

    #[test]
    fn gateway_disabled_omits_gateway_keys() {
        let profile = base_profile();
        let out = generate(&profile);
        assert!(!out.contains("gatewayhostname"));
    }

    #[test]
    fn gateway_enabled_emits_hostname_and_usage_method() {
        let mut profile = base_profile();
        profile.advanced.gateway_usage = GatewayUsage::Always;
        profile.advanced.gateway_hostname = Some("gw.example.com".into());
        let out = generate(&profile);
        assert!(out.contains("gatewayhostname:s:gw.example.com"));
        assert!(out.contains("gatewayusagemethod:i:1"));
    }

    #[test]
    fn audio_mode_maps_to_correct_id() {
        let mut profile = base_profile();
        profile.advanced.audio_mode = AudioMode::None;
        let out = generate(&profile);
        assert!(out.contains("audiomode:i:2"));
    }

    #[test]
    fn clipboard_redirection_defaults_on() {
        let profile = base_profile();
        let out = generate(&profile);
        assert!(out.contains("redirectclipboard:i:1"));
    }
}
