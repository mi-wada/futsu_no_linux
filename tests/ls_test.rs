use assert_cmd::Command;

#[test]
fn test_success_empty_dir() {
    let mut cmd = Command::cargo_bin("ls").unwrap();

    let assert = cmd.arg("hoge").arg("tests/assets/ls/empty").assert();
    assert.success().stdout("\n");
}

#[test]
fn test_success_not_empty() {
    let mut cmd = Command::cargo_bin("ls").unwrap();

    let assert = cmd.arg("tests/assets/ls/3_files").assert();
    assert.success().stdout(".\n..\n1\n3\n2\n\n");
}
