use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// profile_id -> PIDs of currently-running RDP client processes for it.
type SessionsMap = Arc<Mutex<HashMap<String, Vec<u32>>>>;

/// Tracks which RDP client processes are running for which profile.
///
/// This is in-memory *and* persisted to a small JSON file, because the
/// in-memory state alone doesn't survive an app restart — but the spawned
/// mstsc/xfreerdp/sdl-freerdp processes are independent OS processes that
/// keep running fine across an RDP Manager restart. Without persistence,
/// restarting the app would silently "forget" every still-open session:
/// the Dashboard would show it as inactive, Disconnect/jump-to-window
/// would stop working for it, even though the window is still open.
/// On startup we reconcile the persisted list against reality (is each
/// pid still alive?) so stale entries from crashed/closed sessions don't
/// linger forever.
#[derive(Clone)]
pub struct SessionsState {
    map: SessionsMap,
    persist_path: Arc<PathBuf>,
}

impl SessionsState {
    /// Loads persisted sessions (if any) and drops entries whose pid is no
    /// longer alive, then re-saves the reconciled list.
    pub fn load_reconciled(persist_path: PathBuf) -> Self {
        let raw = load_from_disk(&persist_path);
        let mut alive: HashMap<String, Vec<u32>> = HashMap::new();
        for (profile_id, pids) in raw {
            let live_pids: Vec<u32> = pids.into_iter().filter(|&pid| is_process_alive(pid)).collect();
            if !live_pids.is_empty() {
                alive.insert(profile_id, live_pids);
            }
        }
        let state = Self {
            map: Arc::new(Mutex::new(alive)),
            persist_path: Arc::new(persist_path),
        };
        state.save();
        state
    }

    fn save(&self) {
        let map = self.map.lock().unwrap();
        if let Ok(json) = serde_json::to_string(&*map) {
            let _ = std::fs::write(&*self.persist_path, json);
        }
    }

    pub fn mark_started(&self, profile_id: &str, pid: u32) {
        {
            let mut map = self.map.lock().unwrap();
            map.entry(profile_id.to_string()).or_default().push(pid);
        }
        self.save();
    }

    /// Returns true if this was the last active session for the profile
    /// (i.e. the profile just transitioned to inactive).
    pub fn mark_ended(&self, profile_id: &str, pid: u32) -> bool {
        let became_inactive = {
            let mut map = self.map.lock().unwrap();
            let Some(pids) = map.get_mut(profile_id) else {
                return false;
            };
            pids.retain(|&p| p != pid);
            if pids.is_empty() {
                map.remove(profile_id);
                true
            } else {
                false
            }
        };
        self.save();
        became_inactive
    }

    pub fn active_profile_ids(&self) -> Vec<String> {
        self.map.lock().unwrap().keys().cloned().collect()
    }

    pub fn pids_for(&self, profile_id: &str) -> Vec<u32> {
        self.map
            .lock()
            .unwrap()
            .get(profile_id)
            .cloned()
            .unwrap_or_default()
    }
}

fn load_from_disk(path: &Path) -> HashMap<String, Vec<u32>> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

#[cfg(target_os = "macos")]
fn is_process_alive(pid: u32) -> bool {
    // Signal 0 doesn't actually send a signal — kill() just validates the
    // pid exists and is signalable, which is exactly the check we want.
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

#[cfg(target_os = "windows")]
fn is_process_alive(pid: u32) -> bool {
    let output = std::process::Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/NH"])
        .output();
    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).contains(&pid.to_string()),
        Err(_) => false,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn is_process_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}
