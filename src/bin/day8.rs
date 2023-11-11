#![warn(clippy::pedantic)]
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
    grid.enumerate(|x, y| {
        if is_tree_visible(&grid, x, y) {
            visible_count += 1;
        }
    });
    println!("[Part I] There are {visible_count} tree(s) visible from outside the grid");

    // Determine the highest scenic score (part 2)
    let mut highest_scenic_score: u32 = 0;
    grid.enumerate(|x, y| {
        let scenic_score = calc_scenic_score(&grid, x, y);
        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    });
    println!("[Part II] The highest scenic score is {highest_scenic_score}");

    Ok(())
}

// Attempts to read the input as a grid of row data (tree heights)
fn read_tree_grid(reader: &mut impl BufRead) -> RowGrid<u8> {
    let mut tree_grid: Option<RowGrid<u8>> = None;

    // Read each line and parse it as a row of tree heights, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        // Lazy initialization of the grid, we need the first line to determine column width
        let grid = tree_grid.get_or_insert(RowGrid::with_width(line.len()));

        // For each digit on the line, add it as a height value to the grid
        // Since ASCII digits are 0x30 through 0x39 we can just modulo to get the value
        let row: Vec<u8> = line.bytes().map(|digit| (digit % 0x30)).collect();

        grid.push_row(row);
    }

    tree_grid.unwrap()
}

// Determines if a tree is visible at the x/y location within the grid
fn is_tree_visible(grid: &RowGrid<u8>, x: usize, y: usize) -> bool {
    // Assume any perimeter tree is visible
    if x == 0 || x >= grid.width - 1 {
        return true;
    }
    if y == 0 || y >= grid.height() - 1 {
        return true;
    }

    // Get the height of the tree itself
    let tree = match grid.cell(x, y) {
        Some(tree) => *tree,
        None => return false,
    };

    // Get the row that the tree is located within
    let Some(row) = grid.row(y) else { return false };

    // Get the column that the tree is located within
    let Some(column) = grid.column(x) else {
        return false;
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
fn calc_scenic_score(grid: &RowGrid<u8>, x: usize, y: usize) -> u32 {
    let tree = match grid.cell(x, y) {
        Some(val) => *val,
        None => return 0,
    };

    let height = grid.height();
    let width = grid.width;

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
    for right_x in x + 1..width {
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
    for bottom_y in y + 1..height {
        bottom_score += 1;
        match grid.cell(x, bottom_y) {
            Some(other) if *other < tree => {}
            _ => break,
        }
    }

    left_score * right_score * top_score * bottom_score
}
