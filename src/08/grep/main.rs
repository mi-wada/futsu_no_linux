use std::io::{BufRead, BufReader};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let pattern = args.next().unwrap();
    let file_name = args.next().unwrap();

    grep(&pattern, file_name, true, false)
}

fn grep(
    pattern: &str,
    file_name: String,
    case_sensitive: bool,
    show_unmatched_line: bool,
) -> Result<()> {
    let pattern = if case_sensitive {
        pattern.to_string()
    } else {
        format!("(?i){}", pattern)
    };
    let reader = BufReader::new(std::fs::File::open(file_name)?);

    reader.lines().try_for_each(|line| {
        let line = line?;

        let re = Regex::new(&pattern)?;
        if re.is_match(&line) && !show_unmatched_line {
            println!("{}", line);
        }

        if !re.is_match(&line) && show_unmatched_line {
            println!("{}", line);
        }

        Ok(())
    })
}
