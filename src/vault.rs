use std::path::Path;

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use rpassword;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use sha2::Sha256;

use crate::keychain;
use crate::models::Vault;
use crate::session;
use crate::vault_store;

#[derive(Clone)]
pub struct PasswordOptions<'a> {
    pub password: Option<&'a str>,
    pub keychain_account: Option<&'a str>,
    pub keychain_service: &'a str,
    pub vault_path: &'a str,
    pub no_session: bool,
}

impl<'a> PasswordOptions<'a> {
    pub fn new(password: Option<&'a str>, vault_path: &'a str) -> Self {
        Self {
            password,
            keychain_account: None,
            keychain_service: "ownkey",
            vault_path,
            no_session: false,
        }
    }
}

pub fn load_vault(path: &str) -> Result<Vault> {
    load_vault_with_password(path, &PasswordOptions::new(None, path))
}

pub fn save_vault(path: &str, vault: &Vault) -> Result<()> {
    save_vault_with_password(path, vault, &PasswordOptions::new(None, path))
}

pub fn ensure_vault_exists(path: &str) -> Result<()> {
    ensure_vault_exists_with_password(path, &PasswordOptions::new(None, path))
}

pub fn load_vault_with_password(path: &str, opts: &PasswordOptions<'_>) -> Result<Vault> {
    let contents = vault_store::lock_and_read(Path::new(path))
        .with_context(|| "Vault not found. Run `ownkey init` to create a new encrypted vault.")?;

    if let Ok(blob) = serde_json::from_str::<EncryptedVault>(&contents) {
        if !opts.no_session {
            if let Ok(Some(key_bytes)) = session::load(opts.vault_path) {
                if let Ok(vault) = decrypt_vault_with_key(&blob, &key_bytes) {
                    return Ok(vault);
                }
            }
        }

        let pass = get_password(opts)?;
        let salt = general_purpose::STANDARD
            .decode(&blob.salt)
            .with_context(|| "Vault format invalid. Please restore from backup or reinitialize.")?;
        let key_bytes = derive_key_bytes(&pass, &salt);
        if !opts.no_session {
            let _ = session::store(opts.vault_path, &key_bytes);
        }
        return decrypt_vault_with_key(&blob, &key_bytes)
            .with_context(|| "Vault password is incorrect or vault is corrupted.");
    }

    // Fallback: best-effort compatibility for old plain JSON.
    match serde_json::from_str::<Vault>(&contents) {
        Ok(vault) => Ok(vault),
        Err(primary_err) => {
            let value: Value = serde_json::from_str(&contents)
                .with_context(|| "failed to parse vault JSON for migration")?;
            if let Some(items) = value.get("items").and_then(|v| v.as_array()) {
                let mut entries = std::collections::HashMap::new();
                for item in items {
                    if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                        let secret = item
                            .get("secret")
                            .and_then(|s| s.as_str())
                            .unwrap_or_default()
                            .to_string();
                        entries.insert(name.to_string(), secret);
                    }
                }
                return Ok(Vault { entries });
            }
            Err(primary_err).with_context(|| "Vault file appears damaged or truncated. A backup copy may be available.")
        }
    }
}

pub fn save_vault_with_password(path: &str, vault: &Vault, opts: &PasswordOptions<'_>) -> Result<()> {
    let pass = get_password(opts)?;
    let encrypted = encrypt_vault(vault, &pass)?;
    let json = serde_json::to_string_pretty(&encrypted)
        .with_context(|| "failed to serialize encrypted vault")?;
    let salt_bytes = general_purpose::STANDARD
        .decode(&encrypted.salt)
        .with_context(|| "Vault format invalid. Please restore from backup or reinitialize.")?;
    if !opts.no_session {
        let key_bytes = derive_key_bytes(&pass, &salt_bytes);
        let _ = session::store(opts.vault_path, &key_bytes);
    }
    vault_store::lock_and_write(Path::new(path), &json)?;
    Ok(())
}

pub fn ensure_vault_exists_with_password(path: &str, opts: &PasswordOptions<'_>) -> Result<()> {
    let vault_path = Path::new(path);

    if vault_path.exists() {
        return Ok(());
    }

    if let Some(parent) = vault_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let empty_vault = Vault::default();
    let pass = match opts.password {
        Some(p) => p.to_string(),
        None => prompt_new_password()?,
    };
    save_vault_with_password(
        path,
        &empty_vault,
        &PasswordOptions {
            password: Some(&pass),
            keychain_account: opts.keychain_account,
            keychain_service: opts.keychain_service,
            vault_path: opts.vault_path,
            no_session: opts.no_session,
        },
    )?;
    if let Some(account) = opts.keychain_account {
        let _ = keychain::store_password(opts.keychain_service, account, &pass);
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct EncryptedVault {
    salt: String,
    nonce: String,
    ciphertext: String,
}

fn get_password(opts: &PasswordOptions<'_>) -> Result<String> {
    if let Some(p) = opts.password {
        return Ok(p.to_string());
    }
    if let Some(account) = opts.keychain_account {
        if let Ok(Some(pw)) = keychain::retrieve_password(opts.keychain_service, account) {
            return Ok(pw);
        }
    }
    let password =
        rpassword::prompt_password("Enter vault password: ").with_context(|| "password prompt")?;
    Ok(password)
}

fn prompt_new_password() -> Result<String> {
    let first = rpassword::prompt_password("Set a new vault password: ")
        .with_context(|| "password prompt")?;
    let second = rpassword::prompt_password("Confirm password: ")
        .with_context(|| "password confirm prompt")?;
    if first != second {
        anyhow::bail!("passwords do not match");
    }
    Ok(first)
}

fn encrypt_vault(vault: &Vault, password: &str) -> Result<EncryptedVault> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key(password, &salt);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new(&key);
    let plaintext = serde_json::to_vec(vault).with_context(|| "failed to serialize vault")?;
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .map_err(|e| anyhow::anyhow!("encryption failed: {e:?}"))?;

    Ok(EncryptedVault {
        salt: general_purpose::STANDARD.encode(salt),
        nonce: general_purpose::STANDARD.encode(nonce_bytes),
        ciphertext: general_purpose::STANDARD.encode(ciphertext),
    })
}

fn decrypt_vault_with_key(blob: &EncryptedVault, key_bytes: &[u8; 32]) -> Result<Vault> {
    let nonce_bytes = general_purpose::STANDARD
        .decode(&blob.nonce)
        .with_context(|| "invalid nonce encoding")?;
    let ciphertext = general_purpose::STANDARD
        .decode(&blob.ciphertext)
        .with_context(|| "invalid ciphertext encoding")?;
    let cipher = Aes256Gcm::new_from_slice(key_bytes).map_err(|e| anyhow::anyhow!(e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| anyhow::anyhow!("Vault password is incorrect or vault is corrupted."))?;
    let vault: Vault =
        serde_json::from_slice(&plaintext).with_context(|| "Vault format invalid. Please restore from backup or reinitialize.")?;
    Ok(vault)
}

fn derive_key(password: &str, salt: &[u8]) -> aes_gcm::Key<Aes256Gcm> {
    let key = derive_key_bytes(password, salt);
    aes_gcm::Key::<Aes256Gcm>::from_slice(&key).to_owned()
}

fn derive_key_bytes(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}
