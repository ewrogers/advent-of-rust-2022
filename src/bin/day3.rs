#![warn(clippy::pedantic)]
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day3_input.txt")?;
    let reader = BufReader::new(file);

    // Storing each rucksack as a string of "items" (characters)
    let mut rucksacks: Vec<String> = Vec::with_capacity(1000);

    // Attempt to read each line as a rucksack, skip if invalid or empty line
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        rucksacks.push(line);
    }

    // Determine the total priority for all common items in each rucksack (part 1)
    let total_item_priority: u64 = rucksacks
        .iter()
        .map(|sack| {
            // The rucksack is split into two compartments, in equal halves
            let first_compartment = &sack[0..sack.len() / 2];
            let second_compartment = &sack[sack.len() / 2..];
            // Determine the common item and calculate its priority
            match get_common_item(first_compartment, second_compartment) {
                Some(item) => get_item_priority(item),
                None => 0,
            }
        })
        .sum();

    // Determine the total badge priority, chunking rucksacks into groups of three (part 2)
    let total_badge_priority: u64 = rucksacks
        .chunks_exact(3)
        .map(|group| match get_common_badge(group) {
            Some(badge) => get_item_priority(badge),
            None => 0,
        })
        .sum();

    println!("[Part I] Total priority of all items is {total_item_priority}");
    println!("[Part II] Total badge priority is {total_badge_priority}");
    Ok(())
}

// Attempts to get the common item between two "compartment" slices
fn get_common_item(first: &str, second: &str) -> Option<char> {
    first.chars().find(|&item| second.contains(item))
}

// Attempts to get the common badge item between a group of rucksacks
fn get_common_badge(rucksacks: &[String]) -> Option<char> {
    if rucksacks.len() < 2 {
        return None;
    }

    // Using the first rucksack as the reference to all others
    let first_rucksack = &rucksacks[0];
    let other_rucksacks = &rucksacks[1..];

    // If this item was found in all other rucksacks, it's the common badge
    first_rucksack
        .chars()
        .find(|&item| other_rucksacks.iter().all(|sack| sack.contains(item)))
}

// Gets the priority value of an item
fn get_item_priority(item: char) -> u64 {
    // Priority is essentially the one-based index of the letter within the alphabet
    static LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    match LETTERS.find(item) {
        Some(index) => (index + 1) as u64,
        None => 0,
    }
}
