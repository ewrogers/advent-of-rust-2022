#![warn(clippy::pedantic)]

use advent_of_rust_2022::Point;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Sensor {
    pub location: Point,
    pub beacon: Point,
}

impl Display for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            self.location.x, self.location.y, self.beacon.x, self.beacon.y,
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day15_input_example.txt")?;
    let mut reader = BufReader::new(file);

    // Reads the sensor data into
    let sensors = read_sensors(&mut reader);

    for sensor in sensors {
        println!("{sensor}");
    }

    Ok(())
}

fn read_sensors(reader: &mut impl BufRead) -> Vec<Sensor> {
    let mut sensors = Vec::with_capacity(100);

    // Read each line as a scan trace, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => {
                continue;
            }
            Ok(line) => line,
            Err(_) => break,
        };

        // Split the sensor line by the colon character
        let (sensor_str, beacon_str) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("Invalid sensor line: {line}"));

        // Attempt to parse the sensor location part
        let location = parse_point(sensor_str)
            .unwrap_or_else(|| panic!("Invalid sensor location: {sensor_str}"));

        // Attempt to parse the beacon location part
        let beacon = parse_point(beacon_str)
            .unwrap_or_else(|| panic!("Invalid beacon location: {beacon_str}"));

        sensors.push(Sensor { location, beacon });
    }

    sensors
}

// Attempts to parse a point from the slice, assuming both `x=` and `y=` are present
fn parse_point(slice: &str) -> Option<Point> {
    let mut x: Option<i32> = None;
    let mut y: Option<i32> = None;

    // Used to trim out non-numeric characters
    let is_not_numeric = |c| !char::is_numeric(c);

    // Start by splitting by whitespace to find the expression
    for part in slice.split_whitespace() {
        // Try to split by the equal sign to see if we have an expression
        if let Some((lhs, rhs)) = part.split_once('=') {
            // If the left-hand side is X or Y, attempt to parse as a number
            match lhs {
                "x" => {
                    if let Ok(value) = rhs.trim_matches(is_not_numeric).parse::<i32>() {
                        x.replace(value);
                    }
                }
                "y" => {
                    if let Ok(value) = rhs.trim_matches(is_not_numeric).parse::<i32>() {
                        y.replace(value);
                    }
                }
                _ => continue,
            }
        }
    }

    // If both X and Y were found, return a point
    match (x, y) {
        (Some(x), Some(y)) => Some(Point::new(x, y)),
        _ => None,
    }
}
