use std::io::{self, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let file_name = std::env::args().nth(1);

    cat_no_libc(&file_name)
}

fn cat_no_libc(file_name: &Option<String>) -> Result<()> {
    let mut reader: Box<dyn Read> = match file_name {
        Some(file_name) => Box::new(std::fs::File::open(file_name)?),
        None => Box::new(io::stdin()),
    };

    let mut buf = [0u8; 1024];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }

        print!("{}", std::str::from_utf8(&buf[..n])?);
    }

    Ok(())
}
