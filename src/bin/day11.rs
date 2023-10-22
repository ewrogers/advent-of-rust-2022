use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operation {
    None,
    Add(i32),
    MultiplyBy(i32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: Vec<u32>,
    operation: Operation,
    test_divisible_by: i32,
    if_true_target: usize,
    if_false_target: usize,
}

impl Monkey {
    pub fn with_index(index: usize) -> Self {
        Self {
            index,
            items: vec![],
            operation: Operation::None,
            test_divisible_by: 0,
            if_true_target: 0,
            if_false_target: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day11_input.txt")?;
    let mut reader = BufReader::new(file);

    let monkeys = read_monkey_data(&mut reader);

    for monkey in monkeys {
        println!("{:#?}", monkey);
    }

    Ok(())
}

// Attempts to read monkey data from an input file
fn read_monkey_data(reader: &mut impl BufRead) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(10);

    // Read each line and interpret as monkey data
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        // Each data section is colon-separated, so let's split on that
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => {
                println!("Invalid data line: {line}");
                continue;
            }
        };

        // Create a new monkey with the index when we encounter a starting line
        if key.starts_with("Monkey") {
            let index = match key.split_once(' ') {
                Some((_, index_str)) => match index_str.parse::<usize>() {
                    Ok(index) => index,
                    Err(_) => {
                        println!("Invalid monkey index: {index_str}");
                        continue;
                    }
                },
                None => {
                    println!("Invalid monkey definition: {key}");
                    continue;
                }
            };

            monkeys.push(Monkey::with_index(index));
            continue;
        }

        // Assume the last monkey added is the one we are currently updating
        let monkey = match monkeys.last_mut() {
            Some(monkey) => monkey,
            None => {
                println!("No monkey created yet!");
                continue;
            }
        };

        // Update the monkey based on the key and value encountered this line
        match (key, value) {
            ("Starting items", items_str) => {
                // Split by comma and parse each item as an integer value
                let items = items_str
                    .split(',')
                    .map(|item| item.trim().parse::<u32>().unwrap());
                monkey.items.extend(items);
            }
            ("Operation", op_str) => {
                if let Some(operation) = parse_operation(op_str) {
                    monkey.operation = operation;
                }
            }
            ("Test", test_str) => {
                if let Some(divisor) = parse_test_expression(test_str) {
                    monkey.test_divisible_by = divisor;
                }
            }
            ("If true", if_str) => {
                if let Some(index) = parse_target_monkey(if_str) {
                    monkey.if_true_target = index;
                }
            }
            ("If false", if_str) => {
                if let Some(index) = parse_target_monkey(if_str) {
                    monkey.if_false_target = index;
                }
            }
            _ => {
                println!("Unknown key: {key}");
            }
        }
    }

    monkeys
}

// Attempts to parse the operation from a string value
fn parse_operation(string: &str) -> Option<Operation> {
    // Get list of operands by splitting by space character
    let operands: Vec<&str> = string.split(' ').collect();

    // Determine the operation based on the operands
    // Another time Rust pattern makes this much easier!
    match operands[..] {
        ["new", "=", "old", "+", amount] => match amount.parse() {
            Ok(amount) => Some(Operation::Add(amount)),
            Err(_) => {
                println!("Invalid addition: {string}");
                None
            }
        },
        ["new", "=", "old", "*", "old"] => Some(Operation::Square),
        ["new", "=", "old", "*", mul] => match mul.parse() {
            Ok(multiplier) => Some(Operation::MultiplyBy(multiplier)),
            Err(_) => {
                println!("Invalid multiplier: {string}");
                None
            }
        },
        _ => {
            println!("Invalid operation: {string}");
            None
        }
    }
}

// Attempts to parse a test expression as "divisible by X", returning the X
fn parse_test_expression(string: &str) -> Option<i32> {
    match string.split_once("by") {
        Some((_, value_str)) => match value_str.trim().parse() {
            Ok(value) => Some(value),
            Err(_) => {
                println!("Invalid operand value: {value_str}");
                None
            }
        },
        None => {
            println!("Invalid test expression: {string}");
            None
        }
    }
}

// Attempts to parse a test expression as "throws to monkey N", returning the N
fn parse_target_monkey(string: &str) -> Option<usize> {
    match string.split_once("monkey") {
        Some((_, index_str)) => match index_str.trim().parse() {
            Ok(value) => Some(value),
            Err(_) => {
                println!("Invalid index value: {index_str}");
                None
            }
        },
        None => {
            println!("Invalid target expression: {string}");
            None
        }
    }
}
