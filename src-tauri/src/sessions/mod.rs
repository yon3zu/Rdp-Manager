use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// profile_id -> PIDs of currently-running RDP client processes for it.
pub type SessionsMap = Arc<Mutex<HashMap<String, Vec<u32>>>>;

#[derive(Clone, Default)]
pub struct SessionsState(pub SessionsMap);

pub fn mark_started(sessions: &SessionsMap, profile_id: &str, pid: u32) {
    let mut map = sessions.lock().unwrap();
    map.entry(profile_id.to_string()).or_default().push(pid);
}

/// Returns true if this was the last active session for the profile
/// (i.e. the profile just transitioned to inactive).
pub fn mark_ended(sessions: &SessionsMap, profile_id: &str, pid: u32) -> bool {
    let mut map = sessions.lock().unwrap();
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
}

pub fn active_profile_ids(sessions: &SessionsMap) -> Vec<String> {
    sessions.lock().unwrap().keys().cloned().collect()
}

pub fn pids_for(sessions: &SessionsMap, profile_id: &str) -> Vec<u32> {
    sessions
        .lock()
        .unwrap()
        .get(profile_id)
        .cloned()
        .unwrap_or_default()
}
