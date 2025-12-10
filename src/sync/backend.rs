use super::error::SyncError;

pub trait SyncBackend: Send + Sync {
    fn is_logged_in(&self) -> bool;
    fn login(&self, username: Option<&str>) -> Result<(), SyncError>;
    fn logout(&self) -> Result<(), SyncError>;
    fn pull(&self) -> Result<Option<Vec<u8>>, SyncError>;
    fn push(&self, encrypted_blob: &[u8]) -> Result<(), SyncError>;
}
