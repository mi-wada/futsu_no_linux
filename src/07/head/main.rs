use std::io::{self, BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let line_count = std::env::args().nth(1).unwrap().parse()?;

    head(line_count)
}

fn head(line_count: usize) -> Result<()> {
    let reader = BufReader::new(io::stdin());

    for line in reader.lines().take(line_count) {
        println!("{}", line?);
    }

    Ok(())
}
