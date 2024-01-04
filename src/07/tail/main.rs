use std::io::{self, BufRead, BufReader, Read, SeekFrom};

use anyhow::Result;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let line_count = args.next().unwrap().parse()?;
    let file_name = args.next();

    tail(line_count, file_name)
}

fn tail(line_count: usize, file_name: Option<String>) -> Result<()> {
    let reader = {
        let reader: Box<dyn Read> = match file_name {
            Some(file_name) => Box::new(std::fs::File::open(file_name)?),
            None => Box::new(io::stdin()),
        };
        BufReader::new(reader)
    };

    let lines: Vec<_> = reader.lines().collect();
    let total_line_count = lines.len();

    lines
        .into_iter()
        .skip(total_line_count.saturating_sub(line_count))
        .try_for_each(|line| {
            println!("{}", line?);

            Ok(())
        })
}
