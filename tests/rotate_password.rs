use std::path::PathBuf;
use std::process::Command;

use uuid::Uuid;

fn temp_vault_path(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ownkey_rotate_{test_name}_{}.json", Uuid::new_v4()));
    path
}

#[test]
fn rotate_password_changes_access_password() {
    let path = temp_vault_path("basic");
    let path_str = path.to_str().unwrap().to_string();
    let bin = env!("CARGO_BIN_EXE_ownkey");

    // init with old password
    let status = Command::new(bin)
        .args(["init", &path_str, "--password", "oldpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed");

    // add an entry with old password
    let status = Command::new(bin)
        .arg("add")
        .arg("--path")
        .arg(&path_str)
        .arg("--password")
        .arg("oldpw")
        .arg("rotate_key")
        .arg("rotate_value")
        .status()
        .expect("failed to run ownkey add");
    assert!(status.success(), "add should succeed");

    // rotate password from oldpw -> newpw
    let status = Command::new(bin)
        .arg("rotate-password")
        .arg("--path")
        .arg(&path_str)
        .arg("--password")
        .arg("oldpw")
        .arg("--new-password")
        .arg("newpw")
        .status()
        .expect("failed to run ownkey rotate-password");
    assert!(status.success(), "rotate-password should succeed");

    // viewing with old password should now fail
    let output = Command::new(bin)
        .arg("view")
        .arg("--path")
        .arg(&path_str)
        .arg("--no-session")
        .arg("--password")
        .arg("oldpw")
        .arg("rotate_key")
        .output()
        .expect("failed to run ownkey view with old password");
    assert!(
        !output.status.success(),
        "view with old password should not succeed after rotation"
    );

    // viewing with new password should succeed and show the same value
    let output = Command::new(bin)
        .arg("view")
        .arg("--path")
        .arg(&path_str)
        .arg("--no-session")
        .arg("--password")
        .arg("newpw")
        .arg("rotate_key")
        .output()
        .expect("failed to run ownkey view with new password");
    assert!(
        output.status.success(),
        "view with new password should succeed after rotation"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("rotate_value"),
        "view with new password should show the original value"
    );
}
