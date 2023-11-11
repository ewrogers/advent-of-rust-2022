#![warn(clippy::pedantic)]
use advent_of_rust_2022::{find_path, Point, RowGrid};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Terrain {
    StartLocation,
    Goal,
    Height(u8),
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day12_input.txt")?;
    let mut reader = BufReader::new(file);

    // Read the terrain grid from the input file
    let grid = read_terrain_grid(&mut reader);

    // Get the start and goal locations (S -> E)
    let start = match grid.find(|val| val == &Terrain::StartLocation) {
        Some((x, y)) => Point::from_pos(x as i32, y as i32),
        None => panic!("Unable to find start location!"),
    };
    let goal = match grid.find(|val| val == &Terrain::Goal) {
        Some((x, y)) => Point::from_pos(x as i32, y as i32),
        None => panic!("Unable to find goal location!"),
    };

    // Find the path from start to goal and output the number of steps (part 1)
    let path_result = find_path(&start, &goal, |from, to| calc_move_cost(&grid, *from, *to));
    let Some(path) = path_result else {
        panic!("Unable to find path from start to goal!");
    };

    println!(
        "[Part I] This path reaches the goal in {} steps, the fewest possible",
        path.len() - 1
    );

    // Get all the possible starting points (either 'a' or 'S')
    let possible_starts: Vec<Point> = grid
        .find_all(|val| matches!(val, Terrain::StartLocation | Terrain::Height(1)))
        .iter()
        .map(|(x, y)| Point::from_pos(*x as i32, *y as i32))
        .collect();

    // Determine the best (shortest) path from any start location (part 2)
    let mut shortest_path: Option<Vec<Point>> = None;
    for start in possible_starts {
        let path_result = find_path(&start, &goal, |from, to| calc_move_cost(&grid, *from, *to));
        let Some(path) = path_result else { continue };

        if let Some(other_path) = &shortest_path {
            if path.len() < other_path.len() {
                shortest_path.replace(path);
            }
        } else {
            shortest_path.replace(path);
        }
    }

    println!(
        "[Path II] The hiking trail from reaches the goal in {} steps, the fewest possible",
        shortest_path.unwrap().len() - 1
    );

    Ok(())
}

// Calculate the movement cost from one space to another (or None if impossible move)
fn calc_move_cost(grid: &RowGrid<Terrain>, from: Point, to: Point) -> Option<u32> {
    // Out of bounds
    if to.x < 0 || to.y < 0 {
        return None;
    }

    // Determine the neighbor's height, or None if not valid
    let neighbor_height = match grid.cell(to.x as usize, to.y as usize) {
        // The goal is always considered the 'z' height (letter 26)
        Some(Terrain::Goal) => 26,
        Some(Terrain::Height(height)) => *height as i32,
        _ => return None,
    };

    // Determine the current space's height
    let this_height = match grid.cell(from.x as usize, from.y as usize) {
        // The start is always considered the 'a' height (letter 1)
        Some(Terrain::StartLocation) => 1,
        Some(Terrain::Height(height)) => *height as i32,
        _ => return None,
    };

    // Only allow a +1 height increase per move
    if neighbor_height <= this_height + 1 {
        Some(neighbor_height as u32)
    } else {
        None
    }
}

// Visualizes the path taken on the grid, useful for debugging
#[allow(dead_code)]
fn visualize_path(path: &[Point], width: usize, height: usize) {
    let mut cells = vec!['_'; width * height];

    for i in 0..path.len() {
        let current = path.get(i);
        let next = path.get(i + 1);

        let char = match (current, next) {
            (Some(from), Some(to)) if from.x < to.x => '>',
            (Some(from), Some(to)) if from.x > to.x => '<',
            (Some(from), Some(to)) if from.y < to.y => 'v',
            (Some(from), Some(to)) if from.y > to.y => '^',
            (Some(_), None) => 'E',
            _ => '?',
        };

        if let Some(point) = current {
            let index = (point.y as usize * width) + point.x as usize;
            cells[index] = char;
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("[{}]", cells[y * width + x])
        }
        println!()
    }
}

// Reads the terrain grid from the input file
fn read_terrain_grid(reader: &mut impl BufRead) -> RowGrid<Terrain> {
    let mut terrain_grid: Option<RowGrid<Terrain>> = None;

    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        // Lazy initialize the terrain grid, with as many columns as the first row of chars
        let grid = terrain_grid.get_or_insert(RowGrid::with_width(line.len()));

        // Map the letters to height values 1-26 ('a' is 0x61 .. 'z' is 0x7A)
        // If 'S', use as start location, and 'E' as goal location
        let row = line
            .chars()
            .map(|val| match val {
                'S' => Terrain::StartLocation,
                'E' => Terrain::Goal,
                _ => Terrain::Height((val as u8) - 0x60),
            })
            .collect();

        grid.push_row(row);
    }

    terrain_grid.unwrap()
}
