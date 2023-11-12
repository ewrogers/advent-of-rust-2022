use advent_of_rust_2022::{Point, UniformGrid};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Default)]
enum Terrain {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Terrain::Air => '.',
            Terrain::Rock => '#',
            Terrain::Sand => 'o',
        };
        write!(f, "{char}")
    }
}

struct ScanTrace {
    pub points: Vec<Point>,
}

impl Display for ScanTrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, point) in self.points.iter().enumerate() {
            if index > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{},{}", point.x, point.y)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day14_input_example.txt")?;
    let mut reader = BufReader::new(file);

    // Load all the scan traces from input file
    let scan_traces = read_scan_traces(&mut reader);

    // Add all of the traces as rock paths to the 2D terrain grid
    let mut terrain_grid: UniformGrid<Terrain> = UniformGrid::new(1000, 20);
    for scan_trace in &scan_traces {
        add_rock_path(&mut terrain_grid, scan_trace);
    }

    Ok(())
}

// Attempts to read the scan trace data
fn read_scan_traces(reader: &mut impl BufRead) -> Vec<ScanTrace> {
    let mut scans: Vec<ScanTrace> = Vec::with_capacity(100);

    // Read each line as a scan trace, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => {
                continue;
            }
            Ok(line) => line,
            Err(_) => break,
        };

        // Split the line by the `->` string and attempt to parse as point collection
        let points: Vec<Point> = line
            .split("->")
            .map(|pt| {
                let (x, y) = pt
                    .split_once(',')
                    .unwrap_or_else(|| panic!("Invalid point: {pt}"));
                let x = x
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid X coordinate: {x}"));
                let y = y
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid Y coordinate: {y}"));

                Point::new(x, y)
            })
            .collect();

        scans.push(ScanTrace { points })
    }

    scans
}

fn add_rock_path(grid: &mut UniformGrid<Terrain>, path: &ScanTrace) {
    if path.points.is_empty() {
        return;
    }

    let mut points = VecDeque::from(path.points.clone());
    let start_point = points.pop_front().unwrap();

    let mut x = start_point.x;
    let mut y = start_point.y;

    while let Some(next_point) = points.pop_front() {}
}
