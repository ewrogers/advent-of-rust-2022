#![warn(clippy::pedantic)]
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

// Potentially recursive data structure, as lists can contain lists
#[derive(Debug, Clone)]
enum PacketData {
    Integer(i32),
    List(Vec<PacketData>),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::Integer(value) => write!(f, "{}", value),
            PacketData::List(items) => {
                write!(f, "[")?;
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

// Pair of left and right packets to compare against
#[derive(Debug)]
struct PacketPair(PacketData, PacketData);

impl Display for PacketPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.0, self.1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day13_input_example.txt")?;
    let mut reader = BufReader::new(file);

    let pairs = read_packet_pairs(&mut reader);

    for pair in pairs {
        println!("{}", pair);
        println!();
    }

    Ok(())
}

// Attempts to read all the packet pairs from the input file
fn read_packet_pairs(reader: &mut impl BufRead) -> Vec<PacketPair> {
    let mut pairs: Vec<PacketPair> = Vec::with_capacity(1000);
    let mut lines: Vec<String> = vec![];

    // Read each line as a pair, starting a new pair when we reach an empty line
    for line in reader.lines() {
        match line {
            Ok(line) if line.is_empty() => {
                lines.clear();
                continue;
            }
            Ok(line) => lines.push(line),
            Err(_) => break,
        };

        // Need at least two lines to form the pair
        if lines.len() < 2 {
            continue;
        }

        // Attempt to parse the left/right packet pair
        let left_packet = parse_packet_line(&lines[0]);
        let right_packet = parse_packet_line(&lines[1]);

        // If successful, add to the list of pairs otherwise output an error
        match (left_packet, right_packet) {
            (Some(left), Some(right)) => pairs.push(PacketPair(left, right)),
            (None, _) => println!("Invalid left packet: {}", &lines[0]),
            (_, None) => println!("Invalid right packet: {}", &lines[1]),
        }
    }

    pairs
}

// Attempts to parse the line as packet data, returning None if unable to parse
fn parse_packet_line(str: &str) -> Option<PacketData> {
    let cursor = Cursor::new(str);
    let mut reader = BufReader::new(cursor);

    // Parse the entire line as a list, skipping the opening bracket
    reader.consume(1);
    read_packet_list(&mut reader)
}

// Attempts to read a list, advancing the position (returns None if parsing failed)
fn read_packet_list(reader: &mut impl BufRead) -> Option<PacketData> {
    let mut items: Vec<PacketData> = vec![];

    // Peek at the next character, we may not want to consume it before passing to a nested reader
    while let Ok(buf) = reader.fill_buf() {
        if buf.is_empty() {
            break;
        }

        match buf[0] as char {
            // Start of a nested list, read it recursively
            '[' => {
                reader.consume(1);
                match read_packet_list(reader) {
                    Some(list) => items.push(list),
                    None => return None,
                }
            }
            // End of the list, return the finished list
            ']' => {
                reader.consume(1);
                return Some(PacketData::List(items));
            }
            ',' => reader.consume(1),
            // If encountered a digit, attempt to parse a packet integer
            c if c.is_ascii_digit() => match read_packet_integer(reader) {
                Some(integer) => items.push(integer),
                None => return None,
            },
            c => {
                println!("Unexpected token: {c}");
                return None;
            }
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
                return match digits.parse() {
                    Ok(number) => Some(PacketData::Integer(number)),
                    Err(_) => {
                        println!("Invalid integer: {digits}");
                        None
                    }
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
