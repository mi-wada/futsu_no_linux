use std::io::Read;

use anyhow::{anyhow, Result};
use libc::{EACCES, ENOENT, STDIN_FILENO};

fn main() -> Result<()> {
    let file_name = {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            None
        } else {
            Some(args.get(1).unwrap().clone())
        }
    };

    cat_no_libc(&file_name)
}

fn cat_no_libc(file_name: &Option<String>) -> Result<()> {
    let file = match file_name {
        Some(file_name) => {
            let res = std::fs::File::open(file_name);
            match res {
                Ok(file) => file,
                Err(e) => match e.raw_os_error().unwrap() {
                    EACCES => return Err(anyhow!("No permission")),
                    ENOENT => return Err(anyhow!("File not found")),
                    _ => return Err(anyhow!("Failed to open file")),
                },
            }
        }
        None => std::fs::File::open("/dev/stdin")?,
    };

    let mut buf_reader = std::io::BufReader::new(file);

    let mut buf = [0u8; 1024];
    while let Ok(n) = buf_reader.read(&mut buf) {
        if n == 0 {
            break;
        }

        let buf = &buf[..n];

        // translate buf to String
        let buf = std::str::from_utf8(buf)?;
        print!("{buf}");
    }

    Ok(())
}
