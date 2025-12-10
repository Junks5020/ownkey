use std::fs;
use std::path::PathBuf;

use directories::BaseDirs;

use super::backend::SyncBackend;
use super::error::SyncError;
use crate::vault_store;

/// File-based sync backend.
///
/// login: ensure the remote file exists (creating parent directories as needed).
/// push: copy the local vault file to the remote path.
/// pull: copy the remote file back to the local path and return its contents.
pub struct FileSyncBackend {
    local_path: PathBuf,
    remote_path: PathBuf,
}

impl FileSyncBackend {
    pub fn new(local_path: PathBuf, remote_path: PathBuf) -> Self {
        FileSyncBackend {
            local_path,
            remote_path,
        }
    }

    /// Construct a backend using the default local vault path and a default
    /// remote path under `~/.ownkey/remote_vault.json`.
    pub fn new_default() -> Result<Self, SyncError> {
        let local_str = vault_store::default_vault_path()
            .map_err(|e| SyncError::PushFailed(format!("failed to resolve local vault path: {e}")))?;
        let local_path = PathBuf::from(local_str);

        let base_dirs = BaseDirs::new()
            .ok_or_else(|| SyncError::PushFailed("cannot resolve home directory for file sync".into()))?;
        let remote_path = base_dirs.home_dir().join(".ownkey").join("remote_vault.json");

        Ok(FileSyncBackend::new(local_path, remote_path))
    }
}

impl SyncBackend for FileSyncBackend {
    fn is_logged_in(&self) -> bool {
        self.remote_path.exists()
    }

    fn login(&self, _username: Option<&str>) -> Result<(), SyncError> {
        if let Some(parent) = self.remote_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| SyncError::PushFailed(format!("failed to create remote dir: {e}")))?;
        }
        if !self.remote_path.exists() {
            // If a local vault exists, use it as the initial remote copy; otherwise create an empty file.
            if self.local_path.exists() {
                fs::copy(&self.local_path, &self.remote_path).map_err(|e| {
                    SyncError::PushFailed(format!("failed to seed remote vault from local: {e}"))
                })?;
            } else {
                fs::write(&self.remote_path, b"{}")
                    .map_err(|e| SyncError::PushFailed(format!("failed to create remote vault: {e}")))?;
            }
        }
        Ok(())
    }

    fn logout(&self) -> Result<(), SyncError> {
        // No-op for file-based backend.
        Ok(())
    }

    fn pull(&self) -> Result<Option<Vec<u8>>, SyncError> {
        if !self.remote_path.exists() {
            return Ok(None);
        }

        if let Some(parent) = self.local_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| SyncError::PullFailed(format!("failed to create local dir: {e}")))?;
        }

        fs::copy(&self.remote_path, &self.local_path)
            .map_err(|e| SyncError::PullFailed(format!("failed to copy remote to local: {e}")))?;

        let contents = fs::read(&self.local_path)
            .map_err(|e| SyncError::PullFailed(format!("failed to read local vault after pull: {e}")))?;
        Ok(Some(contents))
    }

    fn push(&self, _encrypted_blob: &[u8]) -> Result<(), SyncError> {
        if !self.local_path.exists() {
            return Err(SyncError::PushFailed(format!(
                "local vault does not exist at {}",
                self.local_path.display()
            )));
        }

        if let Some(parent) = self.remote_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| SyncError::PushFailed(format!("failed to create remote dir: {e}")))?;
        }

        fs::copy(&self.local_path, &self.remote_path)
            .map_err(|e| SyncError::PushFailed(format!("failed to copy local vault to remote: {e}")))?;

        Ok(())
    }
}

