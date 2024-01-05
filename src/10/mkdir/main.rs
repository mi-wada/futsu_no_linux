use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let dir_name = std::env::args().nth(1).unwrap();
    let recursive = match std::env::args().nth(2) {
        Some(arg) => arg == "-p",
        None => false,
    };

    mkdir(&dir_name, recursive)
}

fn mkdir(dir_name: &str, recursive: bool) -> Result<()> {
    unsafe {
        if !recursive && libc::mkdir(dir_name.as_ptr() as _, libc::S_IRWXU) == -1 {
            return Err(anyhow!("Failed to create dir"));
        }

        if recursive {
            let mut path = std::path::PathBuf::new();
            for component in dir_name.split('/') {
                path.push(component);
                if libc::mkdir(path.to_str().unwrap().as_ptr() as _, libc::S_IRWXU) == -1 {
                    match std::io::Error::last_os_error().raw_os_error().unwrap() {
                        libc::EEXIST => {}
                        _ => return Err(anyhow!("Failed to create dir")),
                    }
                }
            }
        }
    }
    Ok(())
}
