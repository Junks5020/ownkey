use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use directories::BaseDirs;
use fd_lock::RwLock;
use rand::RngCore;

const TMP_SUFFIX: &str = ".tmp";

pub fn default_vault_path() -> Result<String> {
    let base = BaseDirs::new().ok_or_else(|| anyhow!("cannot resolve home directory"))?;
    let dir = base.home_dir().join(".ownkey");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("vault.json").to_string_lossy().to_string())
}

fn backup_path() -> Result<PathBuf> {
    let base = BaseDirs::new().ok_or_else(|| anyhow!("cannot resolve home directory"))?;
    let dir = base.home_dir().join(".ownkey").join("backups");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("vault.json.bak"))
}

pub fn lock_and_read(path: &Path) -> Result<String> {
    prepare_parent(path)?;
    let mut open = OpenOptions::new();
    open.create(true).read(true).write(true);
    #[cfg(unix)]
    {
        open.mode(0o600);
    }
    let file = open
        .open(path)
        .with_context(|| format!("failed to open vault at {}", path.display()))?;
    let mut lock = RwLock::new(file);
    let _guard = lock
        .write()
        .map_err(|_| anyhow!("Vault is currently in use by another ownkey process. Please try again later."))?;
    enforce_permissions(path)?;

    let mut f = OpenOptions::new().read(true).open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .with_context(|| "failed to read vault file")?;
    Ok(buf)
}

pub fn lock_and_write(path: &Path, contents: &str) -> Result<()> {
    prepare_parent(path)?;
    let mut open = OpenOptions::new();
    open.create(true).read(true).write(true);
    #[cfg(unix)]
    {
        open.mode(0o600);
    }
    let file = open
        .open(path)
        .with_context(|| format!("failed to open vault at {}", path.display()))?;
    let mut lock = RwLock::new(file);
    let _guard = lock
        .write()
        .map_err(|_| anyhow!("Vault is currently in use by another ownkey process. Please try again later."))?;
    enforce_permissions(path)?;
    atomic_write(path, contents)?;
    let backup = backup_path()?;
    atomic_write(&backup, contents)?;
    Ok(())
}

fn prepare_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut suffix_bytes = [0u8; 8];
    rng.fill_bytes(&mut suffix_bytes);
    let tmp_name = format!(
        "{}{}{:x}",
        path.file_name().and_then(|s| s.to_str()).unwrap_or("vault.json"),
        TMP_SUFFIX,
        u64::from_be_bytes(suffix_bytes)
    );
    let tmp_path = path
        .parent()
        .map(|p| p.join(&tmp_name))
        .unwrap_or_else(|| PathBuf::from(tmp_name.clone()));

    {
        let mut open = OpenOptions::new();
        open.create(true).write(true).truncate(true);
        #[cfg(unix)]
        {
            open.mode(0o600);
        }
        let mut f = open
            .open(&tmp_path)
            .with_context(|| format!("failed to open temp file {}", tmp_path.display()))?;
        f.write_all(contents.as_bytes())
            .with_context(|| "failed to write temp vault")?;
        f.sync_all().with_context(|| "failed to sync temp vault")?;
    }

    fs::rename(&tmp_path, path)
        .with_context(|| format!("failed to replace vault at {}", path.display()))?;

    if let Some(parent) = path.parent() {
        if let Ok(dir) = OpenOptions::new().read(true).open(parent) {
            let _ = dir.sync_all();
        }
    }
    enforce_permissions(path)?;
    Ok(())
}

pub fn enforce_permissions(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::fs::Permissions;
        let metadata = fs::metadata(path)?;
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o600 {
            eprintln!("Warning: vault file permissions are insecure (expected 600). Fixing...");
            let perm = Permissions::from_mode(0o600);
            fs::set_permissions(path, perm)?;
        }
    }
    #[cfg(windows)]
    {
        // Not enforced; document unsupported.
    }
    Ok(())
}

pub fn restore_backup() -> Result<()> {
    let path = PathBuf::from(default_vault_path()?);
    let backup = backup_path()?;
    if !backup.exists() {
        return Err(anyhow!("No backup found at {}", backup.display()));
    }
    let contents = fs::read_to_string(&backup)
        .with_context(|| format!("failed to read backup {}", backup.display()))?;
    atomic_write(&path, &contents)?;
    Ok(())
}
