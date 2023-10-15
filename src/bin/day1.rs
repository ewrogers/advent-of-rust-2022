use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day1_input.txt")?;
    let reader = BufReader::new(file);

    // Storing each elf as a number of total calories
    let mut elves: Vec<u32> = vec![];
    let mut calories: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        // New line means the start of a new elf
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
            continue;
        }

        if let Ok(value) = line.parse::<u32>() {
            calories += value;
        }
    }

    // If there was an elf in progress before EOF, include them too
    if calories > 0 {
        elves.push(calories);
    }

    // Sort elves by calories, in descending order (highest to lowest)
    elves.sort_by(|a, b| b.cmp(a));

    // Since our elves are sorted, we can grab the first for top calories
    let max_calories = elves.first().unwrap_or(&0);
    println!("[Part I] The top elf has {} calories", max_calories);

    // Take the top three elves and sum their calories together
    let top_three_sum: u32 = elves.iter().take(3).sum();
    println!(
        "[Part II] The top three elves have a total of {} calories",
        top_three_sum
    );
    Ok(())
}
