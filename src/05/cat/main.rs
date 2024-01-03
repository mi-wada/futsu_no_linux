use anyhow::{anyhow, Result};
use libc::{EACCES, ENOENT};

fn main() -> Result<()> {
    let file_name = {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err(anyhow!("Please specify a file name"));
        }

        args.get(1).unwrap().clone()
    };

    cat(&file_name)
}

fn cat(file_name: &str) -> Result<()> {
    let fd = unsafe { libc::open(file_name.as_ptr() as _, libc::O_RDONLY) };
    if fd == -1 {
        match std::io::Error::last_os_error().raw_os_error().unwrap() {
            EACCES => return Err(anyhow!("No permission")),
            ENOENT => return Err(anyhow!("File not found")),
            _ => {}
        }
    }

    let mut buf = [0u8; 1024];
    loop {
        let n = unsafe { libc::read(fd, buf.as_mut_ptr() as _, buf.len()) };
        if n < 0 {
            return Err(anyhow!("Failed to read file"));
        }
        if n == 0 {
            break;
        }

        let buf = &buf[..n as usize];
        unsafe {
            libc::write(libc::STDOUT_FILENO, buf.as_ptr() as _, buf.len());
        }
    }

    unsafe {
        libc::close(fd);
    }

    Ok(())
}
