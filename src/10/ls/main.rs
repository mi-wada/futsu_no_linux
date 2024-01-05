use anyhow::{anyhow, Result};
use libc::{closedir, opendir, readdir, ENOENT};

fn main() -> Result<()> {
    let dir_name = std::env::args().nth(1).unwrap();

    ls(&dir_name)?;
    println!();

    Ok(())
}

fn ls(dir_name: &str) -> Result<()> {
    let dir = unsafe { opendir(dir_name.as_ptr() as _) };

    if dir.is_null() {
        return match std::io::Error::last_os_error().raw_os_error().unwrap() {
            ENOENT => Ok(()),
            _ => Err(anyhow!("Failed to open dir")),
        };
    }

    loop {
        let entry = unsafe { readdir(dir) };
        if entry.is_null() {
            break;
        }

        let name = unsafe { (*entry).d_name.as_ptr() };
        let name = unsafe { std::ffi::CStr::from_ptr(name).to_str()? };
        println!("{}", name);
    }

    unsafe {
        closedir(dir);
    }

    Ok(())
}
