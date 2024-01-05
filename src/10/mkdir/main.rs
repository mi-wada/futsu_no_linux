use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let dir_name = std::env::args().nth(1).unwrap();

    mkdir(&dir_name)
}

fn mkdir(dir_name: &str) -> Result<()> {
    unsafe {
        if libc::mkdir(dir_name.as_ptr() as _, libc::S_IRWXU) == -1 {
            return Err(anyhow!("Failed to create dir"));
        }
    }
    Ok(())
}
