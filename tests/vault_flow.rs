use std::fs;
use std::path::PathBuf;
use std::process::Command;

use uuid::Uuid;

fn temp_vault_path(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ownkey_test_{test_name}_{}.json", Uuid::new_v4()));
    path
}

#[test]
fn init_creates_empty_vault_file() {
    let path = temp_vault_path("init");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");

    assert!(status.success(), "init should exit successfully");
    assert!(
        path.exists(),
        "init should create a vault file at the provided path"
    );

    let contents = fs::read_to_string(&path).expect("vault file should be readable");
    assert!(
        contents.contains("ciphertext"),
        "vault file should be encrypted"
    );
}

#[test]
fn add_persists_new_item() {
    let path = temp_vault_path("add");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before add");

    let add_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("add")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .arg("123123")
        .status()
        .expect("failed to run ownkey add");
    assert!(add_status.success(), "add should exit successfully");

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("view")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .output()
        .expect("failed to run ownkey view");
    assert!(output.status.success(), "view after add should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("123123"), "value should be visible after add");
}

#[test]
fn list_shows_items_after_add() {
    let path = temp_vault_path("list");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before list");

    let add_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("add")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .arg("123123")
        .status()
        .expect("failed to run ownkey add");
    assert!(add_status.success(), "add should exit successfully");

    let list_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("list")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .status()
        .expect("failed to run ownkey list");
    assert!(list_status.success(), "list should exit successfully");
}

#[test]
fn view_displays_existing_item() {
    let path = temp_vault_path("view");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before view");

    let add_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("add")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .arg("123123")
        .status()
        .expect("failed to run ownkey add");
    assert!(add_status.success(), "add should exit successfully");

    let item_key = "gj_key";

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("view")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg(item_key)
        .output()
        .expect("failed to run ownkey view");
    assert!(
        output.status.success(),
        "view should exit successfully for existing item"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("123123"),
        "view output should include value"
    );
}

#[test]
fn delete_removes_item() {
    let path = temp_vault_path("delete");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before delete");

    let add_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("add")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .arg("123123")
        .status()
        .expect("failed to run ownkey add");
    assert!(add_status.success(), "add should exit successfully");

    let del_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("delete")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .status()
        .expect("failed to run ownkey delete");
    assert!(
        del_status.success(),
        "delete should exit successfully for existing item"
    );

    let view_output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("view")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .output()
        .expect("failed to run ownkey view after delete");
    assert!(
        !view_output.status.success(),
        "view after delete should not succeed"
    );
}

#[test]
fn search_finds_matching_item() {
    let path = temp_vault_path("search");
    let status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .args(["init", path.to_str().unwrap(), "--password", "testpw"])
        .status()
        .expect("failed to run ownkey init");
    assert!(status.success(), "init should succeed before search");

    let add_status = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("add")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj_key")
        .arg("123123")
        .status()
        .expect("failed to run ownkey add");
    assert!(add_status.success(), "add should exit successfully");

    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("search")
        .arg("--path")
        .arg(path.to_str().unwrap())
        .arg("--password")
        .arg("testpw")
        .arg("gj")
        .output()
        .expect("failed to run ownkey search");
    assert!(
        output.status.success(),
        "search should exit successfully when matches exist"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("gj_key"),
        "search output should include matching key name"
    );
}
