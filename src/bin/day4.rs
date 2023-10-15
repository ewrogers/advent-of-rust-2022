use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct SectorRange {
    start: u32,
    end: u32,
}

impl SectorRange {
    pub fn parse(string: &str) -> Option<Self> {
        let (start, end) = match string.split_once('-') {
            Some(tuple) => tuple,
            None => return None,
        };

        let start = match start.parse::<u32>() {
            Ok(value) => value,
            Err(_) => return None,
        };

        let end = match end.parse::<u32>() {
            Ok(value) => value,
            Err(_) => return None,
        };

        Some(SectorRange { start, end })
    }
}

struct Assignment(SectorRange, SectorRange);

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day4_input.txt")?;
    let reader = BufReader::new(file);

    let mut assignments: Vec<Assignment> = Vec::with_capacity(1000);

    // Attempt to read each line as an assignment, skip if invalid or empty line
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        // Attempt to split into first and second range, skip if invalid
        let (first_range, second_range) = match line.split_once(',') {
            Some(tuple) => tuple,
            None => continue,
        };

        // Attempt to parse the first range, skip if invalid
        let first_range = match SectorRange::parse(first_range) {
            Some(range) => range,
            None => continue,
        };

        // Attempt to parse the second range, skip if invalid
        let second_range = match SectorRange::parse(second_range) {
            Some(range) => range,
            None => continue,
        };

        let assignment = Assignment(first_range, second_range);
        assignments.push(assignment);
    }

    // Determine the number of fully overlapped assignments (part 1)
    let fully_overlapped_count = assignments
        .iter()
        .filter(|a| is_fully_overlapped(&a.0, &a.1))
        .count();

    // Determine the number of partially overlapped assignments (part 2)
    let partially_overlapped_count = assignments
        .iter()
        .filter(|a| is_partially_overlapped(&a.0, &a.1))
        .count();

    println!("[Part I] There are {fully_overlapped_count} fully overlapped assignments");
    println!("[Part II] There are {partially_overlapped_count} partially overlapped assignments");
    Ok(())
}

// Determines if either A or B can fully overlap one another
fn is_fully_overlapped(a: &SectorRange, b: &SectorRange) -> bool {
    (a.start >= b.start && a.end <= b.end) || (b.start >= a.start && b.end <= a.end)
}

// Determines if either A or B are partially overlapped
fn is_partially_overlapped(a: &SectorRange, b: &SectorRange) -> bool {
    // Ranges overlap if there exists a number 'N' which exists in both ranges:
    // x1 <= N <= x2  AND  y1 <= N <= y2
    // Can also be written as:
    // x1 <= y2 && y1 <= x2
    (a.start <= b.end) && (a.end >= b.start)
}
