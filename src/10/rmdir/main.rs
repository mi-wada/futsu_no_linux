use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let dir_name = std::env::args().nth(1).unwrap();

    rmdir(&dir_name)
}

fn rmdir(dir_name: &str) -> Result<()> {
    unsafe {
        if libc::rmdir(dir_name.as_ptr() as _) == -1 {
            return Err(anyhow!("Failed to remove dir"));
        }
    }
    Ok(())
}
