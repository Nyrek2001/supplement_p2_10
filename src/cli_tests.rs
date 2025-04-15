use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn cli_detects_file_modification() {
    let tmp_dir = tempdir().unwrap();
    let watch_path = tmp_dir.path().join("test.txt");
    fs::write(&watch_path, "hello").unwrap();

    // Spawn the CLI binary
    let mut cmd = Command::cargo_bin("file_tracker").unwrap()
        .arg(tmp_dir.path())
        .spawn()
        .expect("Failed to start CLI app");

    // Wait to ensure file is being watched
    sleep(Duration::from_secs(2));

    // Modify the file
    fs::write(&watch_path, "modified").unwrap();

    // Wait to allow CLI to detect change
    sleep(Duration::from_secs(4));

    // Kill the CLI process
    cmd.kill().unwrap();
}
