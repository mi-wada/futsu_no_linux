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

#[test]
fn test_success_p() {
    let target_dir_name = format!("tmp/not_exist_dir_{}/mkdir_test", rand::random::<u32>());

    let mut cmd = Command::cargo_bin("mkdir").unwrap();
    let assert = cmd.arg(&target_dir_name).arg("-p").assert();
    assert.success();

    {
        let path = std::path::Path::new(&target_dir_name);
        let mut cmd = Command::cargo_bin("rmdir").unwrap();
        cmd.arg(path.to_str().unwrap()).assert();
        cmd.arg(path.parent().unwrap().to_str().unwrap()).assert();
    }
}
