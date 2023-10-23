use advent_of_rust_2022::{manhattan_distance, ArenaLinkedList};
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
    let mut rope_p1: ArenaLinkedList<PointHistory> =
        ArenaLinkedList::from_vec(vec![PointHistory::default(), PointHistory::default()]);
    for movement in moves.iter() {
        simulate_movement(movement, &mut rope_p1);
    }

    // Determine the number of unique spaces the tail has moved into (part 1)
    let tail_count = rope_p1.last().unwrap().visited.len();
    println!("[Part I] The tail has moved to {tail_count} unique positions");

    // Simulate the rope with 10 nodes (part 2)
    let mut rope_p2: ArenaLinkedList<PointHistory> = ArenaLinkedList::new();
    for _ in 0..10 {
        rope_p2.push(PointHistory::default());
    }
    for movement in moves.iter() {
        simulate_movement(movement, &mut rope_p2);
    }

    // Determine the number of unique spaces the 9th knot has moved into (part 2)
    let ninth_count = rope_p2.last().unwrap().visited.len();
    println!("[Part II] The 9th knot has moved to {ninth_count} unique positions");

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
fn simulate_movement(movement: &MoveSpaces, rope: &mut ArenaLinkedList<PointHistory>) {
    let dir = movement.0;
    let spaces = movement.1;

    // Move the head and then move all following nodes relatively to their previous
    // H <- 1 <- 2 <- 3 <- 4 ... < TAIL
    for _ in 0..spaces {
        // The head moves based on the explicit movement
        let head = rope.get_mut(0).unwrap();
        head.move_dir(&dir);

        for i in 0..rope.len() {
            // We only need the leader's position for the follower
            let leader_pos = match rope.get(i) {
                Some(node) => (node.x, node.y),
                None => break,
            };

            // We need a mutable reference to the follower so we can move them
            let follower = match rope.get_mut(i + 1) {
                Some(node) => node,
                None => break,
            };

            follow_the_leader(follower, leader_pos.0, leader_pos.1);
        }
    }
}

// Moves the follower node towards the leader according to the follow distance rules
fn follow_the_leader(follower: &mut PointHistory, leader_x: i32, leader_y: i32) {
    // If not on the same row or column, there is a diagonal distance
    let diagonal = leader_x != follower.x && leader_y != follower.y;

    // If the follower is only a single space behind (including diagonal), do not move it
    // Otherwise we have to move the follower based on whether it is diagonal or not
    let distance = manhattan_distance(leader_x, leader_y, follower.x, follower.y);
    let max_distance = if diagonal { 3 } else { 2 };
    if distance < max_distance {
        return;
    }

    // Determine which horizontal direction we should move the follower (if any)
    let h_dir = match leader_x - follower.x {
        dx if dx.unsigned_abs() > 0 => {
            if dx > 0 {
                Some(Direction::East)
            } else {
                Some(Direction::West)
            }
        }
        _ => None,
    };

    // Determine which vertical direction we should move the follower (if any)
    let v_dir = match leader_y - follower.y {
        dy if dy.unsigned_abs() > 0 => {
            if dy > 0 {
                Some(Direction::North)
            } else {
                Some(Direction::South)
            }
        }
        _ => None,
    };

    // Combine directions for both horizontal and vertical movement
    let move_dir = match (&h_dir, &v_dir) {
        (Some(h), None) => Some(*h),
        (None, Some(v)) => Some(*v),
        (Some(h), Some(v)) => Some(combine_dir(h, v)),
        _ => None,
    };

    // If the follower needs to move, do so
    if let Some(dir) = move_dir {
        follower.move_dir(&dir);
    }
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
