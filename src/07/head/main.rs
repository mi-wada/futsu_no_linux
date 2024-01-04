use std::io::{self, BufRead, BufReader, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let line_count = std::env::args().nth(1).unwrap().parse()?;
    let file_name = std::env::args().nth(2);

    head(line_count, file_name)
}

fn head(line_count: usize, file_name: Option<String>) -> Result<()> {
    let reader = {
        let reader: Box<dyn Read> = match file_name {
            Some(file_name) => Box::new(std::fs::File::open(file_name)?),
            None => Box::new(io::stdin()),
        };
        BufReader::new(reader)
    };

    for line in reader.lines().take(line_count) {
        println!("{}", line?);
    }

    Ok(())
}
