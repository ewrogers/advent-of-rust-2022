use advent_of_rust_2022::ArenaLinkedList;
use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

#[derive(Debug)]
struct MoveSpaces(Direction, i32);

// Represents a point that keeps track of where it has been previously
#[derive(Debug)]
struct PointHistory {
    x: i32,
    y: i32,
    visited: Vec<(i32, i32)>,
}

impl Default for PointHistory {
    fn default() -> Self {
        Self::with_initial(0, 0)
    }
}

impl PointHistory {
    pub fn with_initial(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            visited: vec![(x, y)],
        }
    }

    pub fn move_dir(&mut self, direction: &Direction) {
        let new_pos = match direction {
            Direction::North => (self.x, self.y + 1),
            Direction::South => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::West => (self.x - 1, self.y),
            Direction::Northeast => (self.x + 1, self.y + 1),
            Direction::Northwest => (self.x - 1, self.y + 1),
            Direction::Southeast => (self.x + 1, self.y - 1),
            Direction::Southwest => (self.x - 1, self.y - 1),
        };

        self.x = new_pos.0;
        self.y = new_pos.1;

        if !self.visited.contains(&new_pos) {
            self.visited.push(new_pos)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day9_input.txt")?;
    let mut reader = BufReader::new(file);

    // Parse the input file as a series of moves we can simulate
    let moves = parse_moves(&mut reader);

    // Simulate the rope with just a head + tail (part 1)
    // We need to use a RefCell here for interior mutability
    let rope_p1: ArenaLinkedList<RefCell<PointHistory>> = ArenaLinkedList::from_vec(vec![
        RefCell::new(PointHistory::default()),
        RefCell::new(PointHistory::default()),
    ]);
    for movement in moves {
        simulate_movement(movement, &rope_p1);
    }

    // Determine the number of unique spaces the tail has moved to (part 1)
    let tail_pos_count = rope_p1.last().unwrap().borrow_mut().visited.len();
    println!("[Part I] The tail has moved to {tail_pos_count} unique positions");

    // TODO: part2 using a rope with 10 knots

    Ok(())
}

// Attempts to parse the input as a series of moves and returns them as a vector
fn parse_moves(reader: &mut impl BufRead) -> Vec<MoveSpaces> {
    let mut moves: Vec<MoveSpaces> = Vec::with_capacity(1000);

    // Read each line as a move with direction and spaces, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        // Split by space and ensure at least two tokens (direction + count)
        let tokens: Vec<&str> = line.split(' ').collect();
        if tokens.len() < 2 {
            println!("Invalid move: {line}");
            continue;
        }

        // Get the direction based on the letter
        let direction = match tokens[0] {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => {
                println!("Invalid direction: {}", tokens[0]);
                continue;
            }
        };

        // Parse the number of spaces as an integer
        let spaces: i32 = match tokens[1].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid number of spaces: {}", tokens[1]);
                continue;
            }
        };

        moves.push(MoveSpaces(direction, spaces));
    }

    moves
}

// Simulates head and tail movement a certain direction and spaces
fn simulate_movement(movement: MoveSpaces, rope: &ArenaLinkedList<RefCell<PointHistory>>) {
    let dir = movement.0;
    let spaces = movement.1;

    // Normally Rust would not allow us to get two mutable references from the `rope` list
    // By wrapping the individual values in RefCell we can borrow from the separately
    // NOTE: This only disables borrow checking compile time, it can still throw at runtime!
    let mut head = rope.first().unwrap().borrow_mut();
    let mut tail = rope.last().unwrap().borrow_mut();

    // TODO: Refactor this to handle more than two items, chain moves backwards from head -> tail

    // Move the head in the direction, X number of spaces
    for _ in 0..spaces {
        head.move_dir(&dir);

        // If not on the same row or column, there is a diagonal distance
        let diagonal = head.x != tail.x && head.y != tail.y;

        // If the tail is only a single space behind (including diagonal), do not move it
        // Otherwise we have to move the tail based on whether it is diagonal or not
        let distance = get_distance(head.x, head.y, tail.x, tail.y);
        let max_distance = if diagonal { 3 } else { 2 };
        if distance < max_distance {
            continue;
        }

        // Determine which horizontal direction we should move the tail (if any)
        let h_dir = match head.x - tail.x {
            dx if dx.unsigned_abs() > 0 => {
                if dx > 0 {
                    Some(Direction::East)
                } else {
                    Some(Direction::West)
                }
            }
            _ => None,
        };

        // Determine which vertical direction we should move the tail (if any)
        let v_dir = match head.y - tail.y {
            dy if dy.unsigned_abs() > 0 => {
                if dy > 0 {
                    Some(Direction::North)
                } else {
                    Some(Direction::South)
                }
            }
            _ => None,
        };

        // Move the tail in the direction, which can be diagonal
        let move_dir = match (&h_dir, &v_dir) {
            (Some(h), None) => Some(*h),
            (None, Some(v)) => Some(*v),
            (Some(h), Some(v)) => Some(combine_dir(h, v)),
            _ => None,
        };

        if let Some(dir) = move_dir {
            tail.move_dir(&dir);
        }
    }
}

// Calculates the Manhattan distance between two points
fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> usize {
    ((x1 - x2).unsigned_abs() + (y1 - y2).unsigned_abs()) as usize
}

// Determine a new direction based on combining a horizontal and vertical direction together
fn combine_dir(horizontal: &Direction, vertical: &Direction) -> Direction {
    match (horizontal, vertical) {
        (Direction::East, Direction::North) => Direction::Northeast,
        (Direction::East, Direction::South) => Direction::Southeast,
        (Direction::West, Direction::North) => Direction::Northwest,
        (Direction::West, Direction::South) => Direction::Southwest,
        _ => panic!("Invalid combination of directions"),
    }
}
