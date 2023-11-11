#![warn(clippy::pedantic)]
use crate::Instruction::MoveCrate;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Instruction {
    MoveCrate {
        count: usize,
        from: usize,
        to: usize,
    },
}

impl Instruction {
    pub fn parse(line: &str) -> Option<Self> {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.first() {
            // Move command requires `count`, `to`, and `from` values
            // EX: move 2 from 5 to 9
            Some(&"move") if tokens.len() >= 6 => {
                // Only create the move command if all values were parsed OK
                match (tokens[1].parse(), tokens[3].parse(), tokens[5].parse()) {
                    (Ok(count), Ok(from), Ok(to)) => Some(MoveCrate { count, from, to }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day5_input.txt")?;
    let mut reader = BufReader::new(file);

    // Parse the initial state of the stack of crates
    let mut stacks_p1 = read_initial_stacks(&mut reader);
    let instructions = read_instructions(&mut reader);

    // Make a deep copy of the initial stacks for part 2
    let mut stacks_p2: Vec<Vec<String>> = stacks_p1.clone();

    for inst in instructions {
        match inst {
            MoveCrate { from, to, count } => {
                // Adjust one-based indexes to zero-based
                let from_index = from - 1;
                let to_index = to - 1;

                // Move crate one at a time (part 1)
                for _ in 0..count {
                    move_single_crate(&mut stacks_p1, from_index, to_index);
                }

                // Move crates as a group (part 2)
                move_grouped_crates(&mut stacks_p2, from_index, to_index, count);
            }
        }
    }

    // Determine the top crate of each stack and output as a single string (part 1 & 2)
    let single_move_order = get_top_crate_order(&stacks_p1);
    let grouped_move_order = get_top_crate_order(&stacks_p2);

    println!("[Part I] The top crates in single move ordering is '{single_move_order}'");
    println!("[Part II] The top crates in grouped move ordering is '{grouped_move_order}'");
    Ok(())
}

// Reads the initial stack states and returns them as a vector of stacks
fn read_initial_stacks(reader: &mut impl BufRead) -> Vec<Vec<String>> {
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(10);
    let mut line = String::with_capacity(100);

    // Read each line until we encounter EOF or the empty line
    while let Ok(count) = reader.read_line(&mut line) {
        if count == 0 || line.trim().is_empty() {
            break;
        }

        // Assumes the `[X] [Y] [Z] ...` format for the line
        for (stack_index, char) in line.chars().skip(1).step_by(4).enumerate() {
            // Ensure the stack exists in the collection
            while stacks.len() <= stack_index {
                stacks.push(Vec::with_capacity(100));
            }

            // Build the stack from the "top-down", newer entries are last out
            // We only accept alphabetic values, ignoring the stack "number" line
            if char.is_ascii_alphabetic() {
                let _ = &stacks[stack_index].insert(0, char.to_string());
            }
        }

        // Clear the line buffer for the next read_line
        line.clear();
    }

    stacks
}

// Reads and parses the instructions as a collection
fn read_instructions(reader: &mut impl BufRead) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(1000);

    // Read each line and parse the instruction, ignore empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        if let Some(instruction) = Instruction::parse(&line) {
            instructions.push(instruction);
        }
    }

    instructions
}

// Moves a single crate from one stack to another
fn move_single_crate(stacks: &mut [Vec<String>], from_index: usize, to_index: usize) {
    if from_index == to_index {
        return;
    }

    if let Some(item) = stacks[from_index].pop() {
        stacks[to_index].push(item);
    }
}

fn move_grouped_crates(
    stacks: &mut [Vec<String>],
    from_index: usize,
    to_index: usize,
    count: usize,
) {
    if from_index == to_index {
        return;
    }

    // Pop all items and store as a group to be moved (preserve order)
    let mut group: Vec<String> = Vec::with_capacity(count);
    for _ in 0..count {
        if let Some(item) = stacks[from_index].pop() {
            group.insert(0, item);
        }
    }

    stacks[to_index].append(&mut group);
}

fn get_top_crate_order(stacks: &[Vec<String>]) -> String {
    stacks
        .iter()
        .fold(String::new(), |mut str, stack| match &stack.last() {
            Some(letter) => {
                str.push_str(letter);
                str
            }
            None => str,
        })
}
