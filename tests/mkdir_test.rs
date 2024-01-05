use assert_cmd::Command;

#[test]
fn test_success() {
    let target_dir_name = format!("tmp/rmdir_test_{}", rand::random::<u32>());

    let mut cmd = Command::cargo_bin("mkdir").unwrap();

    let assert = cmd.arg(&target_dir_name).assert();
    assert.success();

    {
        let mut cmd = Command::cargo_bin("rmdir").unwrap();
        cmd.arg(&target_dir_name).assert();
    }
}
