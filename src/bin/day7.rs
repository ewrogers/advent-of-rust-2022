use advent_of_rust_2022::ArenaTree;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Term {
    List,
    ChangeToRootDirectory,
    ChangeToParentDirectory,
    ChangeDirectory(String),
    FileListing(i64, String),
    DirectoryListing(String),
}

#[derive(Debug, Default, PartialEq)]
enum FileEntry {
    #[default]
    Root,
    File(i64, String, usize), // last param is parent node, allowing same name by parent dir
    Directory(String, usize), // last param is parent node, allowing same name by parent dir
}

// Used in part 2!
const TOTAL_DISK_SPACE: u64 = 70_000_000;
const REQUIRED_DISK_SPACE: u64 = 30_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day7_input.txt")?;
    let mut reader = BufReader::new(file);

    // Use an arena tree to represent the file system, with a root directory node
    let mut file_system: ArenaTree<FileEntry> = ArenaTree::default();
    let root_node = file_system.find_or_add_node(FileEntry::Root);

    // Parse the input as a list of terminal commands
    let commands = parse_terminal(&mut reader);

    let mut current_node = root_node;

    // Evaluate each terminal command and output to build the file system
    for cmd in commands {
        match cmd {
            Term::ChangeToRootDirectory => current_node = root_node,
            Term::ChangeToParentDirectory => {
                current_node = match file_system.nodes[current_node].parent {
                    Some(parent_node) => parent_node,
                    None => root_node,
                };
            }
            Term::ChangeDirectory(target) => {
                // Look for an existing directory with the same name
                let found_node = file_system.find_node_by(
                    |node| matches!(&node.value, FileEntry::Directory(name, parent) if name == &target && parent == &current_node),
                );

                // If we've seen the directory before, switch to it otherwise we need to add it
                current_node = match found_node {
                    Some(dir_node) => dir_node,
                    None => {
                        // Insert the new directory to the tree
                        let dir = FileEntry::Directory(target, current_node);
                        let dir_node = file_system.find_or_add_node(dir);

                        // Set the new directory as a child and make it the current directory
                        file_system.set_parent_child(current_node, dir_node);
                        dir_node
                    }
                };
            }
            Term::DirectoryListing(dirname) => {
                // Add this directory to the current directory
                let dir = FileEntry::Directory(dirname, current_node);
                let dir_node = file_system.find_or_add_node(dir);

                // Set the parent/child relationships with the owner directory
                file_system.set_parent_child(current_node, dir_node);
            }
            Term::FileListing(size, filename) => {
                // Add this file to the current directory
                let file = FileEntry::File(size, filename, current_node);
                let file_node = file_system.find_or_add_node(file);

                // Set the parent/child relationships with the owner directory
                file_system.set_parent_child(current_node, file_node);
            }
            _ => {}
        }
    }

    // Print the file tree for fun visualization
    print_file_tree(&file_system, root_node);
    println!();

    // Get all directories under 100k total size, then sum their total_size (part 1)
    let under_100k_sum: u64 = file_system
        .nodes
        .iter()
        .map(|node| match &node.value {
            FileEntry::Directory(_, _) => calc_total_size(&file_system, node.index),
            _ => 0,
        })
        .filter(|dir_size| *dir_size <= 100_000)
        .sum();

    println!("[Part I] The sum of all directories with 100K or less is {under_100k_sum}");

    // Determine the total used and size needed (part 2)
    let total_used: u64 = calc_total_size(&file_system, root_node);
    let min_needed = REQUIRED_DISK_SPACE - (TOTAL_DISK_SPACE - total_used);

    // Get all directories that are at least the min_needed, pick the smallest
    let dir_size_to_delete = file_system
        .nodes
        .iter()
        .map(|node| match &node.value {
            FileEntry::Directory(_, _) => calc_total_size(&file_system, node.index),
            _ => 0,
        })
        .filter(|dir_size| *dir_size >= min_needed)
        .min();

    match dir_size_to_delete {
        Some(size_to_delete) => {
            println!("[Part II] Can delete directory with size {size_to_delete} to perform upgrade (needed = {min_needed})")
        }
        None => println!("Unable to solve part 2, no size to delete was found!"),
    }

    Ok(())
}

// Attempts to parse the input as a vector of terminal values
fn parse_terminal(reader: &mut impl BufRead) -> Vec<Term> {
    let mut commands: Vec<Term> = Vec::with_capacity(1000);

    // Read each line and parse it as a terminal value, skipping empty lines
    for line in reader.lines() {
        let line = match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => line,
            Err(_) => break,
        };

        let tokens: Vec<&str> = line.split(' ').collect();

        // Parse the possible terminal inputs/outputs
        // This is where Rust pattern matching really shines!
        let command = match tokens[..] {
            ["$", "ls"] => Term::List,
            ["$", "cd", "/"] => Term::ChangeToRootDirectory,
            ["$", "cd", ".."] => Term::ChangeToParentDirectory,
            ["$", "cd", dirname] => Term::ChangeDirectory(dirname.to_string()),
            ["dir", dirname] => Term::DirectoryListing(dirname.to_string()),
            [size_str, filename] => match size_str.parse::<i64>() {
                Ok(size) => Term::FileListing(size, filename.to_string()),
                Err(_) => {
                    println!("Invalid file size: {size_str}");
                    continue;
                }
            },
            _ => {
                println!("Unknown command: {line}");
                continue;
            }
        };

        commands.push(command);
    }

    commands
}

// Recursively prints the file tree to the console
fn print_file_tree(tree: &ArenaTree<FileEntry>, index: usize) {
    tree.traverse(index, &mut |node, depth| {
        let indent = " ".repeat(depth * 2);

        let total_size = calc_total_size(tree, node.index);

        match &node.value {
            FileEntry::Root => println!("{indent}- / (dir, size={total_size})"),
            FileEntry::Directory(dirname, _) => {
                println!("{indent}- {dirname}/ (dir, size={total_size})")
            }
            FileEntry::File(size, filename, _) => {
                println!("{indent}- {filename} (file, size={size})")
            }
        }
    });
}

// Recursively calculates the total size of a directory
fn calc_total_size(tree: &ArenaTree<FileEntry>, index: usize) -> u64 {
    let node = &tree.nodes[index];

    match &node.value {
        FileEntry::File(size, _, _) => *size as u64,
        _ => node
            .children
            .iter()
            .map(|child| calc_total_size(tree, *child))
            .sum(),
    }
}
