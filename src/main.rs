use std::ffi;

use anyhow::{anyhow, bail, Result};

fn main() -> Result<()> {
    let dir_name = ffi::CString::new(std::env::args().nth(1).unwrap())?;

    unsafe {
        if libc::chdir(dir_name.as_ptr()) == -1 {
            bail!("Cannot change dir!");
        }
    };

    Ok(())
}
