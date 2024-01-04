use assert_cmd::Command;

#[test]
fn test_success() {
    let mut cmd = Command::cargo_bin("grep").unwrap();

    let assert = cmd.arg("hoge").arg("tests/assets/grep/hello.txt").assert();
    assert.success().stdout("hogehoge\n");
}

#[test]
fn test_success_2() {
    let mut cmd = Command::cargo_bin("grep").unwrap();

    let assert = cmd
        .arg("hog.*un")
        .arg("tests/assets/grep/hello.txt")
        .assert();
    assert.success().stdout("hogiuuun\nhoguuun\n");
}
