use thiserror::Error;

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
