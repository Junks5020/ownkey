use thiserror::Error;

/// Errors that can occur during sync operations.
/// Some variants are reserved for future backend implementations (e.g., HTTP backend).
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Login is not supported in this backend")]
    LoginUnsupported,

    #[error("Logout is not supported in this backend")]
    LogoutUnsupported,

    #[error("Push failed: {0}")]
    PushFailed(String),

    #[error("Pull failed: {0}")]
    PullFailed(String),

    #[error("Unknown sync error")]
    Unknown,
}
