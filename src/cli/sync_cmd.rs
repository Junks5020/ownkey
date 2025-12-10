use crate::config::{self, Config, SyncProvider};
use crate::sync::backend::SyncBackend;
use crate::sync::noop::NoopSyncBackend;
use crate::sync::file::FileSyncBackend;

pub fn handle_login(username: Option<&str>) {
    let backend = select_backend();
    let _ = backend.login(username);
    println!("Sync login not implemented yet");
}

pub fn handle_logout() {
    let backend = select_backend();
    let _ = backend.logout();
    println!("Sync logout not implemented yet");
}

pub fn handle_sync() {
    let backend = select_backend();
    let _ = backend.pull();
    let _ = backend.push(&[]);
    println!("Sync not implemented yet");
}

fn select_backend() -> Box<dyn SyncBackend> {
    let cfg = config::load_or_init().unwrap_or_else(|err| {
        eprintln!("Warning: failed to load config: {}", err);
        Config::default()
    });

    match cfg.sync_provider {
        SyncProvider::LocalOnly => Box::new(NoopSyncBackend),
        SyncProvider::File => match FileSyncBackend::new_default() {
            Ok(backend) => Box::new(backend),
            Err(err) => {
                eprintln!(
                    "Warning: failed to initialize file sync backend ({err}), falling back to local_only"
                );
                Box::new(NoopSyncBackend)
            }
        },
        SyncProvider::Http => {
            eprintln!("Warning: http sync backend is not implemented yet, falling back to local_only");
            Box::new(NoopSyncBackend)
        },
    }
}
