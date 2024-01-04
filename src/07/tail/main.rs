use std::io::{self, stdin, BufRead, BufReader, Read, Seek, SeekFrom, Write};

use anyhow::Result;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let line_count = args.next().unwrap().parse()?;
    let file_name = args.next();

    seek_tail(line_count, file_name)
}

// fn tail(line_count: usize, file_name: Option<String>) -> Result<()> {
//     let reader = {
//         let reader: Box<dyn Read> = match file_name {
//             Some(file_name) => Box::new(std::fs::File::open(file_name)?),
//             None => Box::new(io::stdin()),
//         };
//         BufReader::new(reader)
//     };

//     let lines: Vec<_> = reader.lines().collect();
//     let total_line_count = lines.len();

//     lines
//         .into_iter()
//         .skip(total_line_count.saturating_sub(line_count))
//         .try_for_each(|line| {
//             println!("{}", line?);

//             Ok(())
//         })
// }

fn seek_tail(line_count: usize, file_name: Option<String>) -> Result<()> {
    let mut file = match file_name {
        Some(file_name) => std::fs::File::open(file_name)?,
        None => {
            let reader = stdin();

            let mut tempfile = tempfile::tempfile()?;
            for line in reader.lines() {
                let line = line? + "\n";
                tempfile.write_all(line.as_bytes())?;
            }
            tempfile.flush()?;
            tempfile
        }
    };

    let mut break_count = 0;

    file.seek(std::io::SeekFrom::End(0))?;
    while let Ok { .. } = file.seek(std::io::SeekFrom::Current(-1)) {
        let mut buf = [0u8; 1];
        file.read_exact(&mut buf)?;
        file.seek(std::io::SeekFrom::Current(-1))?;
        if buf[0] == b'\n' {
            break_count += 1;
        }
        if break_count == line_count + 1 {
            // Skip \n
            file.seek(std::io::SeekFrom::Current(1))?;
            break;
        }
    }

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    s.pop();
    println!("{s}");

    Ok(())
}
