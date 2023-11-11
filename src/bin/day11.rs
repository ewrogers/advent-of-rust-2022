#![warn(clippy::pedantic)]
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

#[derive(Debug)]
enum Operation {
    Add(i64),
    MultiplyBy(i64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    // Need to use Cell/RefCell here for dynamic borrow-checking
    // We know that we will never double-borrow the same monkey's items
    items: RefCell<VecDeque<i64>>,
    operation: Option<Operation>,
    test_divisible_by: u32,
    if_true_target: usize,
    if_false_target: usize,
    inspect_count: Cell<u32>,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            items: RefCell::new(VecDeque::new()),
            operation: None,
            test_divisible_by: 0,
            if_true_target: 0,
            if_false_target: 0,
            inspect_count: Cell::new(0),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day11_input.txt")?;
    let mut reader = BufReader::new(file);

    // Determine the amount of monkey business after 20 rounds, with worry reduction (part 1)
    let monkeys: Vec<Monkey> = read_monkey_data(&mut reader);
    let monkey_business = calc_monkey_business(&monkeys, 20, |worry| worry / 3);
    println!("[Part I] The level of monkey business after 20 rounds is {monkey_business}");

    // Re-parse the file to process part 2
    reader
        .seek(SeekFrom::Start(0))
        .expect("Unable to re-read the file");

    // Determine the amount of monkey business after 10000 rounds, without worry reduction (part 2)
    let monkeys: Vec<Monkey> = read_monkey_data(&mut reader);

    // Part of Chinese-Remainder Theorem
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // M = product of all modulo
    // This can now be used to 'limit' our worry levels while being divisible by all monkeys
    let modulo: i64 = monkeys
        .iter()
        .map(|m| i64::from(m.test_divisible_by))
        .product();

    let monkey_business = calc_monkey_business(&monkeys, 10_000, |worry| worry % modulo);
    println!("[Part II] The level of monkey business after 10000 rounds is {monkey_business}");

    Ok(())
}

// Attempts to calculate the final monkey business after the specific number of rounds
fn calc_monkey_business<F>(monkeys: &[Monkey], rounds: usize, worry_reduction: F) -> i64
where
    F: Fn(i64) -> i64,
{
    for _ in 1..=rounds {
        // Each monkey takes their turn, in order
        for monkey in monkeys {
            // No items, this monkey does nothing this round
            if monkey.items.borrow().is_empty() {
                continue;
            }

            // Evaluate each item and toss to the target monkey
            while let Some(worry) = monkey.items.borrow_mut().pop_front() {
                // Increment the inspect count (behind a Cell)
                let count = monkey.inspect_count.get();
                monkey.inspect_count.set(count + 1);

                // Apply the operation to determine the new worry level
                let worry = match monkey.operation {
                    Some(Operation::Add(amount)) => worry + amount,
                    Some(Operation::MultiplyBy(mul)) => worry * mul,
                    Some(Operation::Square) => worry * worry,
                    _ => worry,
                };

                // Apply the worry reduction formula
                let worry = worry_reduction(worry);

                // Perform the division test and see which monkey gets the item next
                let divisor = i64::from(monkey.test_divisible_by);
                let target = if worry % divisor == 0 {
                    monkey.if_true_target
                } else {
                    monkey.if_false_target
                };

                // Throw to the next monkey based on the test
                if let Some(target_monkey) = monkeys.get(target) {
                    target_monkey.items.borrow_mut().push_back(worry);
                }
            }
        }
    }

    // Get a sorted vector of inspect counts across all monkeys
    let mut inspect_counts: Vec<i64> = monkeys
        .iter()
        .map(|m| i64::from(m.inspect_count.get()))
        .collect();
    inspect_counts.sort_unstable();

    // Take the largest two inspect counts and multiply them together
    inspect_counts.iter().rev().take(2).product()
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
        let (key, value) = if let Some((key, value)) = line.split_once(':') {
            (key.trim(), value.trim())
        } else {
            println!("Invalid data line: {line}");
            continue;
        };

        // Create a new monkey with the index when we encounter a starting line
        if key.starts_with("Monkey") {
            monkeys.push(Monkey::new());
            continue;
        }

        // Assume the last monkey added is the one we are currently updating
        let Some(monkey) = monkeys.last_mut() else {
            println!("No monkey created yet!");
            continue;
        };

        // Update the monkey based on the key and value encountered this line
        match (key, value) {
            ("Starting items", items_str) => {
                // Split by comma and parse each item as a BigInt value
                let items = items_str
                    .split(',')
                    .map(|item| item.trim().parse::<i64>().unwrap());
                monkey.items.borrow_mut().extend(items);
            }
            ("Operation", op_str) => {
                if let Some(operation) = parse_operation(op_str) {
                    monkey.operation = Some(operation);
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
        ["new", "=", "old", "+", amount] => {
            if let Ok(amount) = amount.parse() {
                Some(Operation::Add(amount))
            } else {
                println!("Invalid addition: {string}");
                None
            }
        }
        ["new", "=", "old", "*", "old"] => Some(Operation::Square),
        ["new", "=", "old", "*", mul] => {
            if let Ok(multiplier) = mul.parse() {
                Some(Operation::MultiplyBy(multiplier))
            } else {
                println!("Invalid multiplier: {string}");
                None
            }
        }
        _ => {
            println!("Invalid operation: {string}");
            None
        }
    }
}

// Attempts to parse a test expression as "divisible by X", returning the X
fn parse_test_expression(string: &str) -> Option<u32> {
    if let Some((_, value_str)) = string.split_once("by") {
        if let Ok(value) = value_str.trim().parse() {
            Some(value)
        } else {
            println!("Invalid operand value: {value_str}");
            None
        }
    } else {
        println!("Invalid test expression: {string}");
        None
    }
}

// Attempts to parse a test expression as "throws to monkey N", returning the N
fn parse_target_monkey(string: &str) -> Option<usize> {
    if let Some((_, index_str)) = string.split_once("monkey") {
        if let Ok(value) = index_str.trim().parse() {
            Some(value)
        } else {
            println!("Invalid index value: {index_str}");
            None
        }
    } else {
        println!("Invalid target expression: {string}");
        None
    }
}
