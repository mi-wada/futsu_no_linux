use assert_cmd::Command;

#[test]
fn test_success_stdio() {
    let mut cmd = Command::cargo_bin("head").unwrap();
    let assert = cmd.arg("5").write_stdin("1\n2\n3\n4\n5\n6\n\x1A").assert();

    assert.success().stdout("1\n2\n3\n4\n5\n");
}

#[test]
fn test_success_stdio_big_line_count() {
    let mut cmd = Command::cargo_bin("head").unwrap();
    let assert = cmd.arg("10").write_stdin("1\n2\n3\n4\n5\n6\n\x1A").assert();

    assert.success().stdout("1\n2\n3\n4\n5\n6\n\x1A\n");
}
