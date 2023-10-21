use advent_of_rust_2022::RowGrid;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day8_input.txt")?;
    let mut reader = BufReader::new(file);

    // Parse the input as grid of tree height rows
    let grid = read_tree_grid(&mut reader);

    // Determine tree visibility (part 1)
    let mut visible_count: usize = 0;
    grid.enumerate(|_, (x, y)| {
        if is_tree_visible(&grid, x, y) {
            visible_count += 1;
        }
    });
    println!("[Part I] There are {visible_count} tree(s) visible from outside the grid");

    // Determine the highest scenic score (part 2)
    let mut highest_scenic_score: u32 = 0;
    grid.enumerate(|_, (x, y)| {
        let scenic_score = calc_scenic_score(&grid, x, y);
        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    });
    println!("[Part II] The highest scenic score is {highest_scenic_score}");

    Ok(())
}

// Attempts to read the input as a grid of row data (tree heights)
fn read_tree_grid(reader: &mut impl BufRead) -> RowGrid<u32> {
    let mut vec: Vec<u32> = Vec::with_capacity(100_000);

    let mut width: Option<usize> = None;

    // Read each line and parse it as a row of tree heights, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        // Set the width based on the first line we encounter (assuming uniform grid)
        if width.is_none() {
            width.replace(line.len());
        }

        // For each digit on the line, add it as a height value to the grid
        // Since ASCII digits are 0x30 through 0x39 we can just modulo to get the value
        line.bytes()
            .map(|digit| (digit % 0x30) as u32)
            .for_each(|height| vec.push(height));
    }

    RowGrid::from_vec(vec, width.unwrap_or(0))
}

// Determines if a tree is visible at the x/y location within the grid
fn is_tree_visible(grid: &RowGrid<u32>, x: usize, y: usize) -> bool {
    // Assume any perimeter tree is visible
    if x == 0 || x >= grid.col_count() - 1 {
        return true;
    }
    if y == 0 || y >= grid.row_count() - 1 {
        return true;
    }

    // Get the height of the tree itself
    let tree = match grid.cell(x, y) {
        Some(tree) => *tree,
        None => return false,
    };

    // Get the row that the tree is located within
    let row = match grid.row(y) {
        Some(row) => row,
        None => return false,
    };

    // Get the column that the tree is located within
    let column = match grid.column(x) {
        Some(col) => col,
        None => return false,
    };

    // Check from the left edge, if any tree occludes it
    let left_occluded = row[0..x].iter().any(|other| *other >= tree);

    // Check from the right edge, if any tree occludes it
    let right_occluded = row[x + 1..].iter().any(|other| *other >= tree);

    // Check from the top edge, if any tree occludes it
    let top_occluded = column[0..y].iter().any(|other| **other >= tree);

    // Check from the bottom edge, if any tree occludes it
    let bottom_occluded = column[y + 1..].iter().any(|other| **other >= tree);

    // If any side is NOT occluded, we are visible from that edge
    !left_occluded || !right_occluded || !top_occluded || !bottom_occluded
}

// Attempts to calculate the scenic score from the x/y location
fn calc_scenic_score(grid: &RowGrid<u32>, x: usize, y: usize) -> u32 {
    let tree = match grid.cell(x, y) {
        Some(val) => *val,
        None => return 0,
    };

    let row_count = grid.row_count();
    let col_count = grid.col_count();

    // Walk from tree to left edge to determine the left-side score
    let mut left_score: u32 = 0;
    for left_x in (0..x).rev() {
        left_score += 1;
        match grid.cell(left_x, y) {
            Some(other) if *other < tree => {}
            _ => break,
        }
    }

    // Walk from tree to right edge to determine the right-side score
    let mut right_score: u32 = 0;
    for right_x in x + 1..col_count {
        right_score += 1;
        match grid.cell(right_x, y) {
            Some(other) if *other < tree => {}
            _ => break,
        }
    }

    // Walk from tree to top edge to determine the top-side score
    let mut top_score: u32 = 0;
    for top_y in (0..y).rev() {
        top_score += 1;
        match grid.cell(x, top_y) {
            Some(other) if *other < tree => {}
            _ => break,
        }
    }

    // Walk from tree to bottom edge to determine the bottom-side score
    let mut bottom_score: u32 = 0;
    for bottom_y in y + 1..row_count {
        bottom_score += 1;
        match grid.cell(x, bottom_y) {
            Some(other) if *other < tree => {}
            _ => break,
        }
    }

    left_score * right_score * top_score * bottom_score
}
