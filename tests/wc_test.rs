use std::{
    fs::{set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
};

use assert_cmd::Command;

#[test]
fn test_wc_success_file_specified() {
    let mut cmd = Command::cargo_bin("wc").unwrap();
    let assert = cmd.arg("tests/assets/wc/hello.txt").assert();

    assert.success().stdout("3");
}

#[test]
fn test_wc_success_file_not_specified() {
    let mut cmd = Command::cargo_bin("wc").unwrap();
    let assert = cmd.write_stdin("hogehoge\nhogehoge\n\x1A").assert();

    assert.success().stdout("2");
}

#[test]
fn test_wc_file_not_found() {
    let mut cmd = Command::cargo_bin("wc").unwrap();
    let assert = cmd.arg("tests/assets/wc/notfound").assert();

    assert.failure().stderr("Error: File not found\n");
}

#[test]
fn test_wc_file_no_permission() {
    let no_permission_file_path = "tests/assets/wc/no_permission";
    set_permissions(no_permission_file_path, Permissions::from_mode(0o000)).unwrap();

    let mut cmd = Command::cargo_bin("wc").unwrap();
    let assert = cmd.arg(no_permission_file_path).assert();
    assert.failure().stderr("Error: No permission\n");

    set_permissions(no_permission_file_path, Permissions::from_mode(0o444)).unwrap();
}
