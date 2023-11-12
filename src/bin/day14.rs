use advent_of_rust_2022::{Point, UniformGrid};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Default, Eq, PartialEq)]
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
    let mut terrain_grid: UniformGrid<Terrain> = UniformGrid::new(800, 200);
    for scan_trace in &scan_traces {
        add_rock_path(&mut terrain_grid, scan_trace);
    }

    println!("Terrain Grid");
    println!("{}", "-".repeat(60));
    visualize_grid(&terrain_grid);
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

#[allow(dead_code)]
fn visualize_grid(grid: &UniformGrid<Terrain>) {
    let rocks = grid.find_all(|x| *x == Terrain::Rock);
    let min_x = rocks.iter().map(|&(x, _)| x).min().unwrap_or_default();
    let max_x = rocks.iter().map(|&(x, _)| x).max().unwrap_or_default();
    let max_y = rocks.iter().map(|&(_, y)| y).max().unwrap_or_default();

    print!("     ");
    for x in min_x..=max_x {
        print!("{}", (x / 100) % 10);
    }
    println!();

    print!("     ");
    for x in min_x..=max_x {
        print!("{}", (x / 10) % 10);
    }
    println!();

    print!("     ");
    for x in min_x..=max_x {
        print!("{}", x % 10);
    }
    println!();

    for y in 0..=max_y {
        print!(" {y: >3} ");
        for x in min_x..=max_x {
            // Sand spawns at 500,0
            if x == 500 && y == 0 {
                print!("+");
            } else {
                let terrain = grid.cell(x, y).unwrap_or(&Terrain::Air);
                print!("{terrain}");
            }
        }
        println!();
    }
}

fn add_rock_path(grid: &mut UniformGrid<Terrain>, path: &ScanTrace) {
    if path.points.is_empty() {
        return;
    }

    let mut points = VecDeque::from(path.points.clone());
    let start_point = points.pop_front().unwrap();

    let mut x = start_point.x;
    let mut y = start_point.y;

    while let Some(next_point) = points.pop_front() {
        grid.set_cell(x as usize, y as usize, Terrain::Rock);

        while x != next_point.x {
            grid.set_cell(x as usize, y as usize, Terrain::Rock);
            if next_point.x > x {
                x += 1;
            } else {
                x -= 1;
            }
        }

        while y != next_point.y {
            grid.set_cell(x as usize, y as usize, Terrain::Rock);
            if next_point.y > y {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }

    grid.set_cell(x as usize, y as usize, Terrain::Rock);
}
