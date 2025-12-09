use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

const SESSION_TTL: Duration = Duration::from_secs(300);

#[derive(Serialize, Deserialize)]
struct SessionData {
    vault_path: String,
    key_b64: String,
    expires_at: u64,
}

fn session_path() -> Result<PathBuf> {
    let base = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("cannot resolve home directory"))?;
    let dir = base.home_dir().join(".ownkey");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("session"))
}

pub fn load(vault_path: &str) -> Result<Option<[u8; 32]>> {
    let path = session_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(&path).with_context(|| "failed to read session cache")?;
    let data: SessionData =
        serde_json::from_str(&contents).with_context(|| "failed to parse session cache")?;

    if data.vault_path != vault_path {
        return Ok(None);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();
    if now > data.expires_at {
        return Ok(None);
    }

    let bytes = general_purpose::STANDARD
        .decode(&data.key_b64)
        .with_context(|| "invalid session key encoding")?;
    let mut key = [0u8; 32];
    if bytes.len() == 32 {
        key.copy_from_slice(&bytes);
        Ok(Some(key))
    } else {
        Ok(None)
    }
}

pub fn store(vault_path: &str, key: &[u8; 32]) -> Result<()> {
    let path = session_path()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();
    let expires_at = now + SESSION_TTL.as_secs();

    let data = SessionData {
        vault_path: vault_path.to_string(),
        key_b64: general_purpose::STANDARD.encode(key),
        expires_at,
    };
    let json = serde_json::to_string(&data)?;
    fs::write(path, json).with_context(|| "failed to write session cache")?;
    Ok(())
}
