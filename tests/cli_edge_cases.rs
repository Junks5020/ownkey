use std::fs;
use std::path::PathBuf;
use std::process::Command;

use uuid::Uuid;

fn temp_vault_path(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ownkey_edge_{test_name}_{}.json", Uuid::new_v4()));
    path
}

#[test]
fn list_succeeds_on_empty_vault() {
    let path = temp_vault_path("empty_list");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before list");

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("list")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .output()
        .expect("failed to run ownkey list");
    assert!(
        output.status.success(),
        "list should succeed even when vault is empty"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.trim().is_empty(),
        "list on empty vault should produce no output"
    );
}

#[test]
fn view_nonexistent_key_returns_error() {
    let path = temp_vault_path("view_missing");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before view");

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("view")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("missing_key")
        .output()
        .expect("failed to run ownkey view");
    assert!(
        !output.status.success(),
        "view should fail for missing key"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No entry found for key missing_key"),
        "error message should mention missing key"
    );
}

#[test]
fn wrong_password_is_rejected() {
    let path = temp_vault_path("wrong_pw");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before add/view");

    let add_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("add")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("secret_key")
        .arg("secret_value")
        .status()
        .expect("failed to run ownkey add");
    assert!(add_status.success(), "add should succeed before wrong-password view");

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("view")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--no-session")
        .arg("--password")
        .arg("wrongpw")
        .arg("secret_key")
        .output()
        .expect("failed to run ownkey view with wrong password");
    assert!(
        !output.status.success(),
        "view with wrong password should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Vault password is incorrect or vault is corrupted"),
        "error message should indicate incorrect password or corrupted vault"
    );
}

#[test]
fn corrupted_vault_file_causes_failure() {
    let path = temp_vault_path("corrupted_file");
    let path_str = path.to_str().unwrap().to_string();

    fs::write(&path, "this is not a valid vault file")
        .expect("should be able to write corrupted contents");

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("list")
        .arg("--path")
        .arg(&path_str)
        .arg("--password")
        .arg("testpw")
        .output()
        .expect("failed to run ownkey list on corrupted file");
    assert!(
        !output.status.success(),
        "list on corrupted vault should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Vault file appears damaged or truncated")
            || stderr.contains("failed to parse vault JSON"),
        "error message should indicate corrupted vault file"
    );
}
