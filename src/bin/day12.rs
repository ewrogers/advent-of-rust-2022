use advent_of_rust_2022::RowGrid;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day12_input.txt")?;
    let mut reader = BufReader::new(file);

    Ok(())
}

fn read_terrain_grid(reader: &mut impl BufRead) -> RowGrid<u32> {
    let mut terrain_grid: Option<RowGrid<u32>> = None;

    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        let grid = terrain_grid.get_or_insert(RowGrid::with_width(line.len()));
    }

    terrain_grid.unwrap()
}
