use super::backend::SyncBackend;
use super::error::SyncError;

pub struct NoopSyncBackend;

impl SyncBackend for NoopSyncBackend {
    fn is_logged_in(&self) -> bool {
        false
    }

    fn login(&self, _u: Option<&str>) -> Result<(), SyncError> {
        Ok(())
    }

    fn logout(&self) -> Result<(), SyncError> {
        Ok(())
    }

    fn pull(&self) -> Result<Option<Vec<u8>>, SyncError> {
        Ok(None)
    }

    fn push(&self, _b: &[u8]) -> Result<(), SyncError> {
        Ok(())
    }
}
