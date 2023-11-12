use advent_of_rust_2022::Point;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Terrain {
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

#[derive(Debug, Default)]
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

    let scan_traces = read_scan_traces(&mut reader);

    for scan_trace in &scan_traces {
        println!("{scan_trace}");
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

                Point::from_pos(x, y)
            })
            .collect();

        scans.push(ScanTrace { points })
    }

    scans
}
