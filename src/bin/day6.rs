#![warn(clippy::pedantic)]
use std::error::Error;
use std::fs::File;
use std::io::Read;

const PACKET_HEADER_SIZE: usize = 4;
const MESSAGE_HEADER_SIZE: usize = 14;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data/day6_input.txt")?;
    let mut message: Vec<u8> = Vec::with_capacity(4096);

    // Read entire message from the input file
    file.read_to_end(&mut message)
        .expect("Unable to read input file");

    // Find the start of the packet (part 1)
    let Some(start_of_packet) = find_start_marker(&message, PACKET_HEADER_SIZE) else {
        println!("Unable to find start of packet");
        return Ok(());
    };

    // Find the start of the message (part 2)
    let Some(start_of_message) = find_start_marker(&message, MESSAGE_HEADER_SIZE) else {
        println!("Unable to find start of message");
        return Ok(());
    };

    println!("[Part I] First packet starts after {start_of_packet} characters");
    println!("[Part II] First message starts after {start_of_message} characters");
    Ok(())
}

// Attempts to find the start of a packet or message within some data block
fn find_start_marker(data: &[u8], header_size: usize) -> Option<usize> {
    let mut scan_buffer: Vec<u8> = Vec::with_capacity(header_size + 1);

    // Scan the entire block for the start marker
    for (pos, value) in data.iter().enumerate() {
        scan_buffer.insert(0, *value);

        // Ensure we read the entire header size before detecting start
        if (pos + 1) < header_size {
            continue;
        }

        // Remove old data out of the sliding window
        while scan_buffer.len() > header_size {
            scan_buffer.pop();
        }

        // If all unique values, we have found the start marker
        if all_are_unique(&scan_buffer) {
            return Some(pos + 1);
        }
    }

    None
}

// Determines if all items within a vector are unique
fn all_are_unique(data: &Vec<u8>) -> bool {
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }

            if data[i] == data[j] {
                return false;
            }
        }
    }
    true
}
