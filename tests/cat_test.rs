use std::{
    fs::{set_permissions, File, Permissions},
    os::unix::fs::PermissionsExt,
};

use assert_cmd::Command;
use libc::chmod;

#[test]
fn test_cat_success() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    let assert = cmd.arg("tests/assets/cat/hello.txt").assert();

    assert.success().stdout("hello\n");
}

#[test]
fn test_cat_arg_missing() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    let assert = cmd.assert();

    assert
        .failure()
        .stderr("Error: Please specify a file name\n");
}

#[test]
fn test_cat_file_not_found() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    let assert = cmd.arg("tests/assets/cat/notfound").assert();

    assert.failure().stderr("Error: File not found\n");
}

#[test]
fn test_cat_file_no_permission() {
    let no_permission_file_path = "tests/assets/cat/no_permission";
    set_permissions(no_permission_file_path, Permissions::from_mode(0o000)).unwrap();

    let mut cmd = Command::cargo_bin("cat").unwrap();
    let assert = cmd.arg(no_permission_file_path).assert();
    assert.failure().stderr("Error: No permission\n");

    set_permissions(no_permission_file_path, Permissions::from_mode(0o444)).unwrap();
}
