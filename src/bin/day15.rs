#![warn(clippy::pedantic)]

use advent_of_rust_2022::{manhattan_distance, Point};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
enum Coverage {
    Uncovered,
    Covered,
    Sensor,
    Beacon,
}

#[derive(Debug, Clone)]
struct Sensor {
    pub location: Point,
    pub beacon: Point,
}

impl Sensor {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn distance_to_beacon(&self) -> i32 {
        manhattan_distance(
            self.location.x,
            self.location.y,
            self.beacon.x,
            self.beacon.y,
        ) as i32
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={} (distance={})",
            self.location.x,
            self.location.y,
            self.beacon.x,
            self.beacon.y,
            self.distance_to_beacon()
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day15_input.txt")?;
    let mut reader = BufReader::new(file);

    // Reads the sensor data from the input file
    let sensors = read_sensors(&mut reader);

    // Determine the cover for the row (part 1)
    let row: i32 = 2_000_000;
    let mut coverage: usize = 0;
    iter_row_coverage(&sensors, row, |_, c| {
        if c == Coverage::Covered {
            coverage += 1;
        }
    });

    println!("[Part I] In row {row}, there are {coverage} position which cannot contain a beacon");
    Ok(())
}

// For a given row, determine the coverage for each cell and call the iterator function
#[allow(clippy::cast_sign_loss)]
fn iter_row_coverage<F>(sensors: &[Sensor], y: i32, mut f: F)
where
    F: FnMut(Point, Coverage),
{
    // Get the left-most sensor by (X - range)
    let leftmost_sensor = sensors
        .iter()
        .min_by(|&s1, &s2| {
            let s1_total = s1.location.x - s1.distance_to_beacon();
            let s2_total = s2.location.x - s2.distance_to_beacon();
            s1_total.cmp(&s2_total)
        })
        .unwrap();

    // Get the right-most sensor by (X + range)
    let rightmost_sensor = sensors
        .iter()
        .max_by(|&s1, &s2| {
            let s1_total = s1.location.x + s1.distance_to_beacon();
            let s2_total = s2.location.x + s2.distance_to_beacon();
            s1_total.cmp(&s2_total)
        })
        .unwrap();

    // Determine the X range based on the left and right sensors
    let min_x = leftmost_sensor.location.x - leftmost_sensor.distance_to_beacon();
    let max_x = rightmost_sensor.location.x + rightmost_sensor.distance_to_beacon();

    for x in min_x..=max_x {
        let pt = Point::new(x, y);

        // Check if there is a sensor on this space
        if sensors
            .iter()
            .any(|s| s.location.x == x && s.location.y == y)
        {
            f(pt, Coverage::Sensor);
            continue;
        }

        // Check if there is a beacon on this space
        if sensors.iter().any(|s| s.beacon.x == x && s.beacon.y == y) {
            f(pt, Coverage::Beacon);
            continue;
        }

        // Check if there is a sensor within range of this
        if sensors.iter().any(|s| {
            let distance = manhattan_distance(s.location.x, s.location.y, x, y);
            distance <= s.distance_to_beacon() as u32
        }) {
            f(pt, Coverage::Covered);
        } else {
            f(pt, Coverage::Uncovered);
        }
    }
}

// Reads the sensor and beacon data file into a vector of sensor data
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
