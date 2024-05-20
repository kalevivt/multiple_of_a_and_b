# Multiple of A and B

This is a Rust application that reads a list of numbers from an input file, performs calculations on them, and writes the results to an output file.

## Usage

Run tests with:

```bash
cargo test
```

Run debug build with:

```bash
cargo run -- <input> <output>
```

Run release build with:

```bash
cargo run --release -- <input> <output>
```

Release build can be built with:

```bash
cargo build --release
```

Where:
- `<input>` is the path to the input file. The input file should contain lines of three numbers each, separated by spaces.
- `<output>` is the path to the output file. This file will be created by the program and will contain the results of the calculations.

## Input File Format

Each line of the input file should contain three numbers: `a`, `b`, and `end`. The program will calculate all numbers from 1 to `end` that are divisible by either `a` or `b`.

## Output File Format

The output file will contain lines of numbers. Each line corresponds to the numbers from 1 to `end` (from the input file) that are divisible by either `a` or `b`. The numbers are sorted in ascending order.

## Dependencies

This project uses the following dependencies:
- `anyhow` for error handling.
