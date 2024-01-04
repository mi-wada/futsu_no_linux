use std::io::{BufRead, BufReader};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let pattern = args.next().unwrap();
    let file_name = args.next().unwrap();

    grep(&pattern, file_name)
}

fn grep(pattern: &str, file_name: String) -> Result<()> {
    let reader = BufReader::new(std::fs::File::open(file_name)?);

    reader.lines().try_for_each(|line| {
        let line = line?;

        let re = Regex::new(pattern)?;
        if re.is_match(&line) {
            println!("{}", line);
        }

        Ok(())
    })
}
