use std::io::{self, BufRead, BufReader, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let line_count = args.next().unwrap().parse()?;
    let file_name = args.next();

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

    reader.lines().take(line_count).try_for_each(|line| {
        println!("{}", line?);

        Ok(())
    })
}
