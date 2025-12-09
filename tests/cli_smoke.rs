use std::process::Command;

// Smoke test to ensure the binary builds and basic help output works.
#[test]
fn displays_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_ownkey"))
        .arg("--help")
        .output()
        .expect("failed to run ownkey --help");

    assert!(
        output.status.success(),
        "ownkey --help should exit successfully"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("ownkey"),
        "help output should mention the binary name"
    );
}
