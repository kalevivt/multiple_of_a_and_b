use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

struct LineNumbers {
    a: u32,
    b: u32,
    end: u32,
}

#[derive(PartialEq)]
struct ResultNumbers {
    end: u32,
    numbers: Vec<u32>,
}

impl fmt::Display for ResultNumbers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numbers_str = self
            .numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}:{}", self.end, numbers_str)
    }
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let path = filename.as_ref();
    let file = File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;
    Ok(io::BufReader::new(file).lines())
}

fn read_items(input: &PathBuf) -> Result<Vec<LineNumbers>> {
    let mut results = Vec::new();
    let lines = read_lines(input).context("Failed to read lines from file")?;

    for (line_num, line) in lines.enumerate() {
        let line = line.with_context(|| format!("Failed to read line {}", line_num + 1))?;
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();

        if numbers.len() == 3 {
            results.push(LineNumbers {
                a: numbers[0],
                b: numbers[1],
                end: numbers[2],
            });
        } else {
            return Err(anyhow::anyhow!("Line {} does not contain exactly 3 numbers", line_num + 1));
        }
    }

    Ok(results)
}

fn is_number_divisible_by(item: &LineNumbers, n: &u32) -> bool {
    n % item.a == 0 || n % item.b == 0
}

fn generate_divisible_numbers(input: &PathBuf) -> Result<Vec<ResultNumbers>> {
    let items = read_items(&input).context("Failed to read items from input file")?;

    let mut results: Vec<ResultNumbers> = items
        .into_iter()
        .map(|item| {
            let numbers: Vec<u32> = (1..=item.end)
                .filter(|n| is_number_divisible_by(&item, n))
                .collect();
            ResultNumbers {
                end: item.end,
                numbers,
            }
        })
        .collect();

    results.sort_by(|a, b| a.end.cmp(&b.end));
    Ok(results)
}

fn write_results(output: &PathBuf, results: Vec<ResultNumbers>) -> Result<()> {
    let file = File::create(&output).with_context(|| format!("Failed to create output file: {:?}", output))?;
    let mut out = BufWriter::new(file);

    for (index, result) in results.iter().enumerate() {
        println!("{}", result);
        writeln!(out, "{}", result).with_context(|| format!("Failed to write result {} to output file", index + 1))?;
    }

    out.flush().context("Failed to flush output buffer")?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        std::process::exit(1);
    }

    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);

    if !input.exists() {
        eprintln!("Input file does not exist: {:?}", input);
        std::process::exit(1);
    }

    let results = generate_divisible_numbers(&input).context("Failed to generate divisible numbers")?;

    write_results(&output, results).context("Failed to write results to output file")?;

    Ok(())
}
