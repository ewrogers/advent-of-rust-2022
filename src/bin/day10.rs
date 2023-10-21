use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NoOp,
    Add(i32),
}

#[derive(Debug)]
struct Cpu {
    register_x: i32,
    cycle: i32,
    busy: u32,
    instruction: Option<Instruction>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register_x: 1,
            cycle: 1,
            busy: 0,
            instruction: None,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.busy > 0
    }

    pub fn signal_strength(&self) -> i32 {
        self.register_x * self.cycle
    }

    pub fn begin_instruction(&mut self, instruction: Instruction) {
        let cycle_delay = match instruction {
            Instruction::NoOp => 1,
            Instruction::Add(_) => 2,
        };

        // Set the CPU as busy executing this instruction for X cycles
        self.busy = cycle_delay;
        self.instruction = Some(instruction);
    }

    pub fn tick(&mut self) {
        // Reduce the busy count this cycle
        self.busy -= 1;

        // If we are no longer busy, finish performing the instruction and clear state
        if !self.is_busy() {
            if let Some(Instruction::Add(amount)) = self.instruction {
                self.register_x += amount
            }
            self.instruction = None;
        }

        // Increment cycle counter for next tick
        self.cycle += 1;
    }
}

// These are the key cycle numbers to report & sum signal strengths
const KEY_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day10_input.txt")?;
    let mut reader = BufReader::new(file);

    // Read all the input data as a queue of instructions
    let mut instructions = parse_instructions(&mut reader);

    // Initialize a CPU that will perform the instructions
    let mut cpu = Cpu::new();

    // Total up the signal strength during key cycles (part 1)
    let mut total_signal_strength = 0;
    while cpu.cycle <= 220 {
        if !cpu.is_busy() && !instructions.is_empty() {
            let next = instructions.pop_front().unwrap();
            cpu.begin_instruction(next);
        }

        if KEY_CYCLES.contains(&cpu.cycle) {
            total_signal_strength += cpu.signal_strength();
            println!(
                "During the {}th cycle, register X has the value {}, so the signal strength is {}",
                cpu.cycle,
                cpu.register_x,
                cpu.signal_strength()
            )
        }

        cpu.tick();
    }
    println!();

    println!("[Part I] The total signal strength is {total_signal_strength}");

    Ok(())
}

// Attempts to parse the input as a queue of executable instructions
fn parse_instructions(reader: &mut impl BufRead) -> VecDeque<Instruction> {
    let mut instructions: VecDeque<Instruction> = VecDeque::with_capacity(1000);

    // Parse each line as a separate instructions, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        let tokens: Vec<&str> = line.split(' ').collect();

        // Parse each instruction
        let instruction = match tokens[..] {
            ["noop"] => Instruction::NoOp,
            ["addx", str_val] => match str_val.parse() {
                Ok(amount) => Instruction::Add(amount),
                Err(_) => {
                    println!("Invalid addx instruction: {line}");
                    continue;
                }
            },
            _ => {
                println!("Invalid instruction: {line}");
                continue;
            }
        };

        instructions.push_back(instruction);
    }
    instructions
}
