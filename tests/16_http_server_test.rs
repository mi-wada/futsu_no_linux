use std::{
    fs::{set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
};

use assert_cmd::Command;

#[test]
fn test_success() {
    let mut cmd = Command::cargo_bin("16_http_server").unwrap();
    let assert = cmd.arg("tests/assets/http_server/valid_request").assert();

    // 無意味なテスト。後で直す。
    assert.success();
}
