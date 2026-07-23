use crate::error::{AppError, AppResult};
use crate::models::profile::{AudioMode, CertTrustBehavior, ScreenMode};
use crate::models::ConnectionProfile;

/// Builds the xfreerdp argument list for a profile. Kept OS-independent
/// (no `#[cfg(target_os = "macos")]`) so it's unit-testable on any platform.
pub fn build(
    profile: &ConnectionProfile,
    password: Option<&str>,
    gateway_password: Option<&str>,
) -> AppResult<Vec<String>> {
    if profile.host.trim().is_empty() {
        return Err(AppError::InvalidInput("host is required".into()));
    }

    let a = &profile.advanced;
    let mut args: Vec<String> = Vec::new();

    args.push(format!("/v:{}:{}", profile.host, profile.port));

    if let Some(username) = &profile.username {
        let value = match &profile.domain {
            Some(domain) if !domain.trim().is_empty() => format!("{domain}\\{username}"),
            _ => username.clone(),
        };
        args.push(format!("/u:{value}"));
    }
    if let Some(password) = password {
        args.push(format!("/p:{password}"));
    }

    match a.screen_mode {
        ScreenMode::Fullscreen => args.push("/f".to_string()),
        ScreenMode::Windowed => {
            let w = a.desktop_width.unwrap_or(1280);
            let h = a.desktop_height.unwrap_or(800);
            args.push(format!("/w:{w}"));
            args.push(format!("/h:{h}"));
        }
    }
    if a.dynamic_resolution {
        args.push("+dynamic-resolution".to_string());
    }

    args.push(format!("/bpp:{}", a.color_depth));

    if a.multi_monitor {
        args.push("/multimon".to_string());
        if let Some(monitors) = &a.selected_monitors {
            if !monitors.trim().is_empty() {
                args.push(format!("/monitors:{monitors}"));
            }
        }
    }

    if a.admin_session {
        args.push("+admin".to_string());
    }

    if a.redirect_drives {
        args.push("+drives".to_string());
    }
    if a.redirect_printers {
        args.push("/printer:".to_string());
    }
    if a.redirect_clipboard {
        args.push("/clipboard".to_string());
    } else {
        args.push("-clipboard".to_string());
    }

    let audio_flag = match a.audio_mode {
        AudioMode::Local => "redirect",
        AudioMode::Remote => "server",
        AudioMode::None => "none",
    };
    args.push(format!("/audio-mode:{audio_flag}"));
    if a.mic_redirection {
        args.push("/microphone".to_string());
    }

    if a.gateway_usage != crate::models::profile::GatewayUsage::None {
        if let Some(gw_host) = &a.gateway_hostname {
            if !gw_host.trim().is_empty() {
                let mut parts = vec![format!("g:{gw_host}")];
                if let Some(port) = a.gateway_port {
                    parts[0] = format!("g:{gw_host}:{port}");
                }
                if let Some(gw_user) = &a.gateway_username {
                    if !gw_user.trim().is_empty() {
                        parts.push(format!("u:{gw_user}"));
                    }
                }
                if let Some(gw_domain) = &a.gateway_domain {
                    if !gw_domain.trim().is_empty() {
                        parts.push(format!("d:{gw_domain}"));
                    }
                }
                if let Some(gw_pass) = gateway_password {
                    parts.push(format!("p:{gw_pass}"));
                }
                args.push(format!("/gateway:{}", parts.join(",")));
            }
        }
    }

    match a.cert_trust_behavior {
        CertTrustBehavior::Deny => args.push("/cert:deny".to_string()),
        CertTrustBehavior::Ignore => args.push("/cert:ignore".to_string()),
        CertTrustBehavior::TrustOnFirstUse => args.push("/cert:tofu".to_string()),
        CertTrustBehavior::Prompt => {} // let xfreerdp prompt interactively
    }

    if let Some(timeout_ms) = a.connection_timeout_ms {
        args.push(format!("/timeout:{timeout_ms}"));
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::profile::{AdvancedSettings, GatewayUsage};

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
    fn builds_host_and_credentials() {
        let profile = base_profile();
        let args = build(&profile, Some("secret"), None).unwrap();
        assert!(args.contains(&"/v:192.168.1.10:3389".to_string()));
        assert!(args.contains(&"/u:alice".to_string()));
        assert!(args.contains(&"/p:secret".to_string()));
    }

    #[test]
    fn windowed_emits_width_height_not_fullscreen() {
        let profile = base_profile();
        let args = build(&profile, None, None).unwrap();
        assert!(args.iter().any(|a| a.starts_with("/w:")));
        assert!(!args.contains(&"/f".to_string()));
    }

    #[test]
    fn fullscreen_emits_f_flag() {
        let mut profile = base_profile();
        profile.advanced.screen_mode = ScreenMode::Fullscreen;
        let args = build(&profile, None, None).unwrap();
        assert!(args.contains(&"/f".to_string()));
        assert!(!args.iter().any(|a| a.starts_with("/w:")));
    }

    #[test]
    fn clipboard_defaults_on() {
        let profile = base_profile();
        let args = build(&profile, None, None).unwrap();
        assert!(args.contains(&"/clipboard".to_string()));
    }

    #[test]
    fn gateway_builds_composite_flag() {
        let mut profile = base_profile();
        profile.advanced.gateway_usage = GatewayUsage::Always;
        profile.advanced.gateway_hostname = Some("gw.example.com".into());
        profile.advanced.gateway_username = Some("bob".into());
        let args = build(&profile, None, Some("gwpass")).unwrap();
        let gw_arg = args
            .iter()
            .find(|a| a.starts_with("/gateway:"))
            .expect("gateway arg present");
        assert!(gw_arg.contains("g:gw.example.com"));
        assert!(gw_arg.contains("u:bob"));
        assert!(gw_arg.contains("p:gwpass"));
    }

    #[test]
    fn no_gateway_when_usage_none() {
        let profile = base_profile();
        let args = build(&profile, None, None).unwrap();
        assert!(!args.iter().any(|a| a.starts_with("/gateway:")));
    }

    #[test]
    fn empty_host_is_rejected() {
        let mut profile = base_profile();
        profile.host = "".into();
        assert!(build(&profile, None, None).is_err());
    }
}
