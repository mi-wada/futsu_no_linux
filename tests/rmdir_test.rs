use assert_cmd::Command;

#[test]
fn test_success() {
    let target_dir_name = format!("tmp/rmdir_test_{}", rand::random::<u32>());

    {
        let mut mkdir = Command::cargo_bin("mkdir").unwrap();
        mkdir.arg(&target_dir_name).assert().success();
    }

    let mut cmd = Command::cargo_bin("rmdir").unwrap();
    let assert = cmd.arg(&target_dir_name).assert();
    assert.success();
}
