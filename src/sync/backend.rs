use super::error::SyncError;

/// Backend trait for sync operations.
/// Currently only `login`, `logout`, `pull`, and `push` are used by CLI commands.
/// `is_logged_in` is reserved for future use (e.g., status checks, conditional sync).
#[allow(dead_code)]
pub trait SyncBackend: Send + Sync {
    fn is_logged_in(&self) -> bool;
    fn login(&self, username: Option<&str>) -> Result<(), SyncError>;
    fn logout(&self) -> Result<(), SyncError>;
    fn pull(&self) -> Result<Option<Vec<u8>>, SyncError>;
    fn push(&self, encrypted_blob: &[u8]) -> Result<(), SyncError>;
}
