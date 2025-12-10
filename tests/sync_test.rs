use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use ownkey::sync::backend::SyncBackend;
use ownkey::sync::noop::NoopSyncBackend;
use ownkey::sync::file::FileSyncBackend;
use uuid::Uuid;

#[test]
fn test_sync_commands_exist() {
    let bin = env!("CARGO_BIN_EXE_ownkey");

    let sync_out = Command::new(bin)
        .arg("sync")
        .output()
        .expect("run ownkey sync");
    assert!(sync_out.status.success());
    let stdout = String::from_utf8_lossy(&sync_out.stdout);
    assert!(stdout.contains("Sync not implemented yet"));

    let login_out = Command::new(bin)
        .arg("login")
        .output()
        .expect("run ownkey login");
    assert!(login_out.status.success());
    let stdout = String::from_utf8_lossy(&login_out.stdout);
    assert!(stdout.contains("Sync login not implemented yet"));

    let logout_out = Command::new(bin)
        .arg("logout")
        .output()
        .expect("run ownkey logout");
    assert!(logout_out.status.success());
    let stdout = String::from_utf8_lossy(&logout_out.stdout);
    assert!(stdout.contains("Sync logout not implemented yet"));
}

#[test]
fn test_sync_backend_trait_basic_behavior() {
    let backend = NoopSyncBackend;
    assert!(!backend.is_logged_in());
    assert!(backend.login(None).is_ok());
    assert!(backend.logout().is_ok());
    assert!(backend.pull().unwrap().is_none());
    assert!(backend.push(b"data").is_ok());
}

#[test]
fn test_sync_flow_invokes_backend() {
    #[derive(Clone)]
    struct FakeBackend {
        login_count: Arc<AtomicUsize>,
        pull_count: Arc<AtomicUsize>,
        push_count: Arc<AtomicUsize>,
    }

    impl SyncBackend for FakeBackend {
        fn is_logged_in(&self) -> bool {
            self.login_count.load(Ordering::SeqCst) > 0
        }

        fn login(&self, _username: Option<&str>) -> Result<(), ownkey::sync::error::SyncError> {
            self.login_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn logout(&self) -> Result<(), ownkey::sync::error::SyncError> {
            Ok(())
        }

        fn pull(&self) -> Result<Option<Vec<u8>>, ownkey::sync::error::SyncError> {
            self.pull_count.fetch_add(1, Ordering::SeqCst);
            Ok(None)
        }

        fn push(&self, _encrypted_blob: &[u8]) -> Result<(), ownkey::sync::error::SyncError> {
            self.push_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    let backend = FakeBackend {
        login_count: Arc::new(AtomicUsize::new(0)),
        pull_count: Arc::new(AtomicUsize::new(0)),
        push_count: Arc::new(AtomicUsize::new(0)),
    };

    assert!(backend.login(None).is_ok());
    assert!(backend.pull().is_ok());
    assert!(backend.push(b"blob").is_ok());

    assert_eq!(backend.login_count.load(Ordering::SeqCst), 1);
    assert_eq!(backend.pull_count.load(Ordering::SeqCst), 1);
    assert_eq!(backend.push_count.load(Ordering::SeqCst), 1);
}

fn temp_paths() -> (PathBuf, PathBuf) {
    let mut dir = std::env::temp_dir();
    let id = Uuid::new_v4();
    dir.push(format!("ownkey_sync_test_{id}"));
    let local = dir.join("local_vault.json");
    let remote = dir.join("remote_vault.json");
    (local, remote)
}

#[test]
fn file_sync_login_creates_remote_and_is_logged_in() {
    let (local, remote) = temp_paths();

    // Ensure parent dir exists for local when we write later, but keep remote missing.
    if let Some(parent) = local.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let backend = FileSyncBackend::new(local.clone(), remote.clone());
    assert!(!backend.is_logged_in(), "should not be logged in before login when remote is missing");

    backend.login(None).expect("login should succeed");
    assert!(remote.exists(), "remote vault should be created on login");
    assert!(backend.is_logged_in(), "should be logged in after login");
}

#[test]
fn file_sync_push_and_pull_round_trip() {
    let (local, remote) = temp_paths();

    if let Some(parent) = local.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Seed local vault content.
    fs::write(&local, b"vault_v1").unwrap();

    let backend = FileSyncBackend::new(local.clone(), remote.clone());
    backend.login(None).expect("login should succeed");

    // Push local -> remote.
    backend.push(b"ignored").expect("push should succeed");
    let remote_contents = fs::read(&remote).expect("remote should be readable");
    assert_eq!(remote_contents, b"vault_v1");

    // Change remote and pull back -> local.
    fs::write(&remote, b"vault_v2").unwrap();
    let pulled = backend.pull().expect("pull should succeed");
    assert!(pulled.is_some(), "pull should return Some for existing remote");
    assert_eq!(pulled.unwrap(), b"vault_v2");

    let local_contents = fs::read(&local).expect("local should be readable after pull");
    assert_eq!(local_contents, b"vault_v2");
}
