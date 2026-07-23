use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// profile_id -> number of currently-running RDP client processes for it.
pub type SessionsMap = Arc<Mutex<HashMap<String, u32>>>;

#[derive(Clone, Default)]
pub struct SessionsState(pub SessionsMap);

pub fn mark_started(sessions: &SessionsMap, profile_id: &str) {
    let mut map = sessions.lock().unwrap();
    *map.entry(profile_id.to_string()).or_insert(0) += 1;
}

/// Returns true if this was the last active session for the profile
/// (i.e. the profile just transitioned to inactive).
pub fn mark_ended(sessions: &SessionsMap, profile_id: &str) -> bool {
    let mut map = sessions.lock().unwrap();
    let Some(count) = map.get_mut(profile_id) else {
        return false;
    };
    *count = count.saturating_sub(1);
    if *count == 0 {
        map.remove(profile_id);
        true
    } else {
        false
    }
}

pub fn active_profile_ids(sessions: &SessionsMap) -> Vec<String> {
    sessions.lock().unwrap().keys().cloned().collect()
}
