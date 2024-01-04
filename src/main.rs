use std::io::Write;

use anyhow::Result;

fn main() -> Result<()> {
    // Write
    // 1
    // 2
    // ...
    // to TOOBIGFILE.txt
    let mut file = std::fs::File::create("./TOOBIGFILE.txt")?;
    for i in 1..=1000000 {
        writeln!(file, "{}", i)?;
    }
    Ok(())
}
