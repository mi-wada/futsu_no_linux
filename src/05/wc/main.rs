use anyhow::{anyhow, Result};
use libc::{EACCES, ENOENT};

fn main() -> Result<()> {
    let file_name = std::env::args().nth(1);

    wc(&file_name)
}

fn wc(file_name: &Option<String>) -> Result<()> {
    let fd = match file_name {
        Some(file_name) => unsafe { libc::open(file_name.as_ptr() as _, libc::O_RDONLY) },
        None => libc::STDIN_FILENO,
    };
    if fd == -1 {
        match std::io::Error::last_os_error().raw_os_error().unwrap() {
            EACCES => return Err(anyhow!("No permission")),
            ENOENT => return Err(anyhow!("File not found")),
            _ => {}
        }
    }

    let mut buf = [0u8; 1024];
    let mut result = 0;
    loop {
        let n = unsafe { libc::read(fd, buf.as_mut_ptr() as _, buf.len()) };
        if n < 0 {
            return Err(anyhow!("Failed to read file"));
        }
        if n == 0 {
            break;
        }

        let buf = &buf[..n as usize];

        result += buf.iter().filter(|c| **c == b'\n').count();
    }

    unsafe {
        libc::write(
            libc::STDOUT_FILENO,
            result.to_string().as_ptr() as _,
            result.to_string().len(),
        );
    }

    unsafe {
        libc::close(fd);
    }

    Ok(())
}
