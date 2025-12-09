use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

use uuid::Uuid;
use directories::BaseDirs;

fn temp_vault() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ownkey_reliability_{}.json", Uuid::new_v4()));
    path
}

#[test]
fn default_path_is_created() {
    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("init")
        .arg("--password")
        .arg("testpw")
        .output()
        .expect("run init");
    assert!(
        output.status.success(),
        "init should succeed for default path"
    );
    let default = BaseDirs::new()
        .unwrap()
        .home_dir()
        .join(".ownkey")
        .join("vault.json");
    assert!(default.exists(), "default vault should exist");
}

#[test]
#[cfg(unix)]
fn permissions_are_600() {
    let path = temp_vault();
    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("init")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .output()
        .expect("run init");
    assert!(output.status.success());
    let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "vault permissions should be 600");
}
