# 🎄 Advent of Rust 2022

The [Advent of Code 2022](https://adventofcode.com/2022) code challenges, written in Rust 🦀.

## 🚫📦 No Crates

These solutions do not use any external crates, only the [Rust Standard Library](https://doc.rust-lang.org/std/).

All "shared" code can be found in the `lib.rs` re-exports.
This includes custom data structures and pathfinding algorithms.

## ⭐️ Getting Started

You can clone the repository to run the solutions:

```shell
$ git clone https://github.com/ewrogers/advent-of-rust-2022
$ cd advent-of-rust-2022

$ cargo run --bin day1
```

Each daily challenge is separated into a separate binary within `src/bin`.

## 🧩 Puzzle Inputs

Each puzzle input is located within the [data](./data) folder, named by the day.

Ex: `data/day1_input.txt`

## 1️⃣ + 2️⃣ Solutions

Solutions for both `Part I` and `Part II` are included within each day folder.

## 👨🏻‍🎨 Code Style

This repository uses `clippy` and `rustfmt` for code style and linting.
The `pedantic` ruleset is used to enforce best practices.
