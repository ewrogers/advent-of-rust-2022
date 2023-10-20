use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day8_input.txt")?;
    let reader = BufReader::new(file);

    Ok(())
}
