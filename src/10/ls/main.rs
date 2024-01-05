use anyhow::{anyhow, Result};
use libc::{opendir, readdir, ENOENT};

fn main() -> Result<()> {
    let dir_name = std::env::args().nth(1).unwrap();

    ls(&dir_name)?;
    println!();

    Ok(())
}

fn ls(dir_name: &str) -> Result<()> {
    unsafe {
        let dir = opendir(dir_name.as_ptr() as _);
        if dir.is_null() {
            return match std::io::Error::last_os_error().raw_os_error().unwrap() {
                ENOENT => Ok(()),
                _ => Err(anyhow!("Failed to open dir")),
            };
        }

        loop {
            let entry = readdir(dir);
            if entry.is_null() {
                break;
            }

            let name = (*entry).d_name.as_ptr();
            let name = std::ffi::CStr::from_ptr(name).to_str()?;
            println!("{}", name);
        }
    }

    Ok(())
}
