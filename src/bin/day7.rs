use advent_of_rust_2022::{ArenaTree, Node};
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
    print_file_tree(&file_system, root_node, 0);
    println!();

    // Get all directories under 100k total size into a vector
    let mut under_100k_vec: Vec<(String, u64)> = Vec::new();
    collect_dirs_into_vec(&file_system, root_node, &mut under_100k_vec, &|(
        node,
        size,
    )| match node.value {
        FileEntry::Directory(_, _) => size <= 100_000,
        _ => false,
    });

    // Sum each directory under 100k (part 1)
    // Verify with `grep -E '\(dir, size=[0-9]{1,5}\)'`
    let under_100k_sum: u64 = under_100k_vec
        .into_iter()
        .map(|(_, total_size)| total_size)
        .sum();

    println!("[Part I] The sum of all directories with 100K or less is {under_100k_sum}");

    // Determine the total used and needed for part 2
    let total_used: u64 = calc_total_size(&file_system, root_node);
    let amount_needed = REQUIRED_DISK_SPACE - (TOTAL_DISK_SPACE - total_used);

    // Get all directories that are at least the amount needed in total size
    let mut big_enough_vec: Vec<(String, u64)> = Vec::new();
    collect_dirs_into_vec(&file_system, root_node, &mut big_enough_vec, &|(
        node,
        size,
    )| match node.value {
        FileEntry::Directory(_, _) => size >= amount_needed,
        _ => false,
    });

    // Select the smallest directory necessary to delete for the required space (part 2)
    let size_to_delete: u64 = big_enough_vec
        .into_iter()
        .map(|(_, size)| size)
        .min()
        .unwrap_or(0);

    println!("[Part II] Can delete directory with size {size_to_delete} to free required {amount_needed}");
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

// Copies each directory with the total size, into a vector (if the dir matches predicate)
fn collect_dirs_into_vec<P>(
    tree: &ArenaTree<FileEntry>,
    index: usize,
    vec: &mut Vec<(String, u64)>,
    predicate: &P,
) -> usize
where
    P: Fn((&Node<FileEntry>, u64)) -> bool,
{
    let node = &tree.nodes[index];
    let total_size = calc_total_size(tree, index);

    let mut push_count: usize = 0;

    match &node.value {
        FileEntry::Directory(name, _) if predicate((node, total_size)) => {
            vec.push((name.clone(), total_size));
            push_count += 1;
        }
        _ => {}
    }

    for child in &node.children {
        push_count += collect_dirs_into_vec(tree, *child, vec, predicate);
    }

    push_count
}

// Recursively prints the file tree to the console
fn print_file_tree(tree: &ArenaTree<FileEntry>, index: usize, indent_count: usize) {
    let node = &tree.nodes[index];
    let total_size = calc_total_size(tree, index);

    let indent = " ".repeat(indent_count);

    match &node.value {
        FileEntry::Root => println!("{indent}- / (dir, size={total_size})"),
        FileEntry::Directory(dirname, _) => {
            println!("{indent}- {dirname}/ (dir, size={total_size})")
        }
        FileEntry::File(size, filename, _) => {
            println!("{indent}- {filename} (file, size={size})")
        }
    }

    for child in &node.children {
        print_file_tree(tree, *child, indent_count + 2)
    }
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
