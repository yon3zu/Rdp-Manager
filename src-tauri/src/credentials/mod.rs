use keyring::Entry;

use crate::error::AppResult;

const SERVICE: &str = "com.rdpmanager.desktop";

fn entry(account: &str) -> AppResult<Entry> {
    Ok(Entry::new(SERVICE, account)?)
}

fn set(account: &str, secret: &str) -> AppResult<()> {
    entry(account)?.set_password(secret)?;
    Ok(())
}

fn get(account: &str) -> AppResult<Option<String>> {
    match entry(account)?.get_password() {
        Ok(secret) => Ok(Some(secret)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn delete(account: &str) -> AppResult<()> {
    match entry(account)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn set_password(profile_id: &str, secret: &str) -> AppResult<()> {
    set(profile_id, secret)
}

pub fn get_password(profile_id: &str) -> AppResult<Option<String>> {
    get(profile_id)
}

pub fn delete_password(profile_id: &str) -> AppResult<()> {
    delete(profile_id)
}

pub fn set_gateway_password(profile_id: &str, secret: &str) -> AppResult<()> {
    set(&format!("{profile_id}:gateway"), secret)
}

pub fn get_gateway_password(profile_id: &str) -> AppResult<Option<String>> {
    get(&format!("{profile_id}:gateway"))
}

pub fn delete_gateway_password(profile_id: &str) -> AppResult<()> {
    delete(&format!("{profile_id}:gateway"))
}

/// Best-effort cleanup on profile delete; ignores missing entries.
pub fn delete_all_for_profile(profile_id: &str) -> AppResult<()> {
    delete_password(profile_id)?;
    delete_gateway_password(profile_id)?;
    Ok(())
}
