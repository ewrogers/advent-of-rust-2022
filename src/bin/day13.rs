#![warn(clippy::pedantic)]

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

// Potentially recursive data structure, as lists can contain lists
#[derive(Debug, Clone, Eq)]
enum PacketData {
    Integer(i32),
    List(Vec<PacketData>),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::Integer(value) => write!(f, "{value}"),
            PacketData::List(items) => {
                write!(f, "[")?;
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        compare_packets(self, other) == Ordering::Equal
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(self, other)
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day13_input.txt")?;
    let mut reader = BufReader::new(file);

    let mut packets = read_packets(&mut reader);
    let mut correct_indices: Vec<usize> = Vec::with_capacity(1000);

    // Display the evaluation of each packet pair
    for (index, packet) in packets.iter().enumerate().step_by(2) {
        let next_packet = packets.get(index + 1).unwrap();
        let pair_number = (index / 2) + 1;

        println!("== Pair {pair_number} == ");
        if is_correct_order(packet, next_packet, None).unwrap_or_default() {
            correct_indices.push(pair_number);
        }
    }
    println!();

    // Add the divider packets (will have to locate these after sorting)
    let divider_packet_2 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let divider_packet_6 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);

    packets.push(divider_packet_2.clone());
    packets.push(divider_packet_6.clone());

    // Sort the packets using our `PartialOrd` and `Ord` implementations
    packets.sort();

    for packet in &packets {
        println!("{packet}");
    }
    println!();

    // Sum all of the correct indices (part 1)
    let part1_sum: usize = correct_indices.iter().sum();
    println!("[Part I] The sum of the correct indices is {part1_sum}");

    // Find the divider packet indices for decoder key (part 2)
    let divider_index_2 = &packets
        .iter()
        .position(|packet| packet == &divider_packet_2)
        .expect("Unable to find divider packet [[2]] in list");

    let divider_index_6 = &packets
        .iter()
        .position(|packet| packet == &divider_packet_6)
        .expect("Unable to find divider packet [[6]] in list");

    let decoder_key = (divider_index_2 + 1) * (divider_index_6 + 1);
    println!("[Part II] The decoder key is {decoder_key}",);
    Ok(())
}

// Attempts to read all the packet from the input file
fn read_packets(reader: &mut impl BufRead) -> Vec<PacketData> {
    let mut packets: Vec<PacketData> = Vec::with_capacity(1000);

    // Read each line as a packet, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => {
                continue;
            }
            Ok(line) => line,
            Err(_) => break,
        };

        // Attempt to parse the packet
        // If successful, add to the list of pairs otherwise output an error
        if let Some(packet) = parse_packet_line(&line) {
            packets.push(packet);
        } else {
            println!("Invalid left packet: {line}");
        }
    }

    packets
}

// Attempts to parse the line as packet data, returning None if unable to parse
fn parse_packet_line(str: &str) -> Option<PacketData> {
    let cursor = Cursor::new(str);
    let mut reader = BufReader::new(cursor);

    // Parse the entire line as a list, skipping the opening bracket
    reader.consume(1);
    let result = read_packet_list(&mut reader);

    // Unwrap the outer list to avoid [[nesting]]
    if let Some(PacketData::List(list)) = result {
        list.first().cloned()
    } else {
        result
    }
}

// Attempts to read a list, advancing the position (returns None if parsing failed)
fn read_packet_list(reader: &mut impl BufRead) -> Option<PacketData> {
    let mut items: Vec<PacketData> = vec![];

    // Peek at the next character, we may not want to consume it before passing to a nested reader
    while let Ok(buf) = reader.fill_buf() {
        if buf.is_empty() {
            break;
        }

        match buf.first().map(|raw| *raw as char) {
            // Start of a nested list, read it recursively
            Some('[') => {
                reader.consume(1);
                match read_packet_list(reader) {
                    Some(list) => items.push(list),
                    None => return None,
                }
            }
            // End of the list, return the finished list
            Some(']') => {
                reader.consume(1);
                return Some(PacketData::List(items));
            }
            Some(',') => reader.consume(1),
            // If encountered a digit, attempt to parse a packet integer
            Some(c) if c.is_ascii_digit() => match read_packet_integer(reader) {
                Some(integer) => items.push(integer),
                None => return None,
            },
            Some(c) => {
                println!("Unexpected token: {c}");
                return None;
            }
            None => return None,
        }
    }

    Some(PacketData::List(items))
}

// Attempts to read an integer, advancing teh position (returns None if parsing failed)
fn read_packet_integer(reader: &mut impl BufRead) -> Option<PacketData> {
    let mut digits = String::new();

    // Peek at the next character, we may not want to consume it before passing to a nested reader
    while let Ok(buf) = reader.fill_buf() {
        if buf.is_empty() {
            break;
        }

        match buf[0] as char {
            // Comma or closing bracket means end of the integer, we can parse the digits
            ',' | ']' => {
                return if let Ok(number) = digits.parse() {
                    Some(PacketData::Integer(number))
                } else {
                    println!("Invalid integer: {digits}");
                    None
                }
            }
            // Append any digits to the buffer
            c if c.is_ascii_digit() => {
                reader.consume(1);
                digits.push(c);
            }
            // Unknown character, return None since it is a parsing error
            c => {
                println!("Invalid digit: {c}");
                return None;
            }
        }
    }

    match digits.parse() {
        Ok(number) => Some(PacketData::Integer(number)),
        Err(_) => None,
    }
}

// Compares two packet data items and determines if they are in the right order
// Very similar to the compare_packets implementation but has additional output for debugging
fn is_correct_order(left: &PacketData, right: &PacketData, indent: Option<usize>) -> Option<bool> {
    let indent_size = indent.unwrap_or_default();
    let indent = " ".repeat(indent_size);
    let indent_more = " ".repeat(indent_size + 2);

    println!("{indent}- Compare {left} vs {right}");

    match (left, right) {
        // Both are lists, compare recursively through each item
        (PacketData::List(left_items), PacketData::List(right_items)) => {
            // Convert into queue since we need to pop from the front
            let mut left_items = VecDeque::from(left_items.clone());
            let mut right_items = VecDeque::from(right_items.clone());

            loop {
                // Take an item from each side and compare
                match (left_items.pop_front(), right_items.pop_front()) {
                    (Some(left), Some(right)) => {
                        if let Some(result) = is_correct_order(&left, &right, Some(indent_size + 2))
                        {
                            return Some(result);
                        }
                    }
                    // Both lists ran out at the same time, indeterminate
                    (None, None) => return None,
                    // Left side ran out of items -- correct order
                    (None, _) => {
                        println!("{indent_more}- Left side ran out of items, so inputs are in the correct order");
                        return Some(true);
                    }
                    // Right side ran out of items -- incorrect order
                    (_, None) => {
                        println!("{indent_more}- Right side ran out of items, so inputs are NOT in the correct order");
                        return Some(false);
                    }
                }
            }
        }
        // Both are integers, compare to see which side is greater
        (PacketData::Integer(left), PacketData::Integer(right)) => match left.cmp(right) {
            Ordering::Less => {
                println!("{indent_more}- Left side is smaller, so inputs are in the correct order");
                Some(true)
            }
            Ordering::Greater => {
                println!(
                    "{indent_more}- Right side is smaller, so inputs are NOT in the correct order"
                );
                Some(false)
            }
            Ordering::Equal => None,
        },
        // The left side is an integer, right side is a list -- convert left to list and retry
        (PacketData::Integer(value), PacketData::List(_)) => {
            println!("{indent_more}- Mixed types; convert left to [{value}] and retry comparison");
            let left = PacketData::List(vec![left.clone()]);
            is_correct_order(&left, right, Some(indent_size + 2))
        }
        // The left side is a list, right side is an integer -- convert right to list and retry
        (PacketData::List(_), PacketData::Integer(value)) => {
            println!("{indent_more}- Mixed types; convert right to [{value}] and retry comparison");
            let right = PacketData::List(vec![right.clone()]);
            is_correct_order(left, &right, Some(indent_size + 2))
        }
    }
}

// Compares packets and returns their relative ordering
fn compare_packets(left: &PacketData, right: &PacketData) -> Ordering {
    match (left, right) {
        // Two lists, compare recursively
        (PacketData::List(left_items), PacketData::List(right_items)) => {
            // Convert into queue since we need to pop from the front
            let mut left_items = VecDeque::from(left_items.clone());
            let mut right_items = VecDeque::from(right_items.clone());

            loop {
                match (left_items.pop_front(), right_items.pop_front()) {
                    (Some(left), Some(right)) => {
                        let result = compare_packets(&left, &right);
                        if result != Ordering::Equal {
                            return result;
                        }
                    }
                    (None, None) => return Ordering::Equal,
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                }
            }
        }
        // Two numbers, compare their values
        (PacketData::Integer(left), PacketData::Integer(right)) => left.cmp(right),
        // The left side is an integer, right side is a list -- convert left to list and retry
        (PacketData::Integer(_), PacketData::List(_)) => {
            let left = PacketData::List(vec![left.clone()]);
            compare_packets(&left, right)
        }
        // The left side is a list, right side is an integer -- convert right to list and retry
        (PacketData::List(_), PacketData::Integer(_)) => {
            let right = PacketData::List(vec![right.clone()]);
            compare_packets(left, &right)
        }
    }
}
