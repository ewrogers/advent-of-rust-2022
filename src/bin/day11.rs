use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

#[derive(Debug)]
enum Operation {
    Add(i32),
    MultiplyBy(i32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    // Need to use Cell/RefCell here for dynamic borrow-checking
    // We know that we will never double-borrow the same monkey's items
    items: RefCell<VecDeque<u32>>,
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
    let file = File::open("data/day11_input_example.txt")?;
    let mut reader = BufReader::new(file);

    // Determine the amount of monkey business after 20 rounds, with worry reduction (part 1)
    let monkeys: Vec<Monkey> = read_monkey_data(&mut reader);
    let monkey_business = calc_monkey_business(&monkeys, 20, true);
    println!("[Part I] The level of monkey business after 20 rounds is {monkey_business}");

    // Re-parse the file to process part 2
    reader
        .seek(SeekFrom::Start(0))
        .expect("Unable to re-read the file");

    // Determine the amount of monkey business after 10000 rounds, without worry reduction (part 2)
    let monkeys: Vec<Monkey> = read_monkey_data(&mut reader);
    let monkey_business = calc_monkey_business(&monkeys, 10_000, false);
    println!("[Part II] The level of monkey business after 1000 rounds is {monkey_business}");

    Ok(())
}

// Attempts to calculate the final monkey business after the specific number of rounds
fn calc_monkey_business(monkeys: &[Monkey], rounds: usize, worry_reduction: bool) -> u64 {
    for round in 1..=rounds {
        // Each monkey takes their turn, in order
        for monkey in monkeys.iter() {
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
                    Some(Operation::Add(amount)) => worry + amount as u32,
                    Some(Operation::MultiplyBy(mul)) => worry * mul as u32,
                    Some(Operation::Square) => worry * worry,
                    _ => worry,
                };

                // Worry is reduced by a factor of 3
                let worry = if worry_reduction { worry / 3 } else { worry };

                // Perform the division test and see which monkey gets the item next
                let divisor = monkey.test_divisible_by;
                let target = match worry % divisor == 0 {
                    true => monkey.if_true_target,
                    false => monkey.if_false_target,
                };

                // Throw to the next monkey based on the test
                if let Some(target_monkey) = monkeys.get(target) {
                    target_monkey.items.borrow_mut().push_back(worry)
                }
            }
        }

        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("== After round {round} ==");
            for (index, monkey) in monkeys.iter().enumerate() {
                println!(
                    "Monkey {index} inspected items {} times",
                    monkey.inspect_count.get()
                );
            }
        }
    }

    // Get a sorted vector of inspect counts across all monkeys
    let mut inspect_counts: Vec<u64> = monkeys
        .iter()
        .map(|m| m.inspect_count.get() as u64)
        .collect();
    inspect_counts.sort();

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
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => {
                println!("Invalid data line: {line}");
                continue;
            }
        };

        // Create a new monkey with the index when we encounter a starting line
        if key.starts_with("Monkey") {
            monkeys.push(Monkey::new());
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
                // Split by comma and parse each item as a BigInt value
                let items = items_str
                    .split(',')
                    .map(|item| item.trim().parse::<u32>().unwrap());
                monkey.items.borrow_mut().extend(items);
            }
            ("Operation", op_str) => {
                if let Some(operation) = parse_operation(op_str) {
                    monkey.operation = Some(operation)
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
fn parse_test_expression(string: &str) -> Option<u32> {
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
