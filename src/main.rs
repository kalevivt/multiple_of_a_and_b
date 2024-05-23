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

    results.sort_by(|a, b| a.numbers.len().cmp(&numbers.len()));
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

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::PathBuf;

    use super::generate_divisible_numbers;
    use super::is_number_divisible_by;
    use super::LineNumbers;
    use super::read_items;
    use super::ResultNumbers;

    fn read_result_numbers_from_file(file_path: &PathBuf) -> Result<Vec<ResultNumbers>, Box<dyn std::error::Error>> {
        let content = read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        let mut result_numbers_vec = Vec::new();

        for line in lines {
            let parts: Vec<&str> = line.split(':').collect();
            let end: u32 = parts[0].parse()?;
            let numbers: Vec<u32> = parts[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            result_numbers_vec.push(ResultNumbers { end, numbers });
        }

        Ok(result_numbers_vec)
    }

    #[test]
    fn test_read_items() {
        let input = PathBuf::from("test_data/input_2_rows.txt");
        let items = read_items(&input).unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_is_number_divisible_by() {
        let item = LineNumbers {
            a: 2,
            b: 3,
            end: 10,
        };

        let expected_results = vec![
            (1, false),
            (2, true),
            (3, true),
            (4, true),
            (5, false),
            (6, true),
            (7, false),
            (8, true),
            (9, true),
            (10, true),
        ];

        for (n, expected) in expected_results {
            assert_eq!(is_number_divisible_by(&item, &n), expected, "Failed for number {}", n);
        }
    }

    #[test]
    fn test_generate_divisible_numbers() {
        // Read the expected results from the comparison file
        let comparison_path = PathBuf::from("test_data/result_2_comparison.txt");
        let expected_results = read_result_numbers_from_file(&comparison_path).unwrap();

        // Call the function with the test input
        let input_path = PathBuf::from("test_data/input_2_rows.txt");
        let actual_results = generate_divisible_numbers(&input_path).unwrap();

        // Compare the output with the expected results
        assert_eq!(actual_results.len(), expected_results.len());
        for (actual, expected) in actual_results.iter().zip(expected_results.iter()) {
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_read_items_incorrect_format() {
        let input = PathBuf::from("test_data/input_incorrect_format.txt");
        let result = read_items(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_items_empty_file() {
        let input = PathBuf::from("test_data/input_empty.txt");
        let items = read_items(&input).unwrap();
        assert!(items.is_empty());
    }

    #[test]
    fn test_read_items_mixed_format() {
        let input = PathBuf::from("test_data/input_mixed_format.txt");
        let result = read_items(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_divisible_numbers_large_numbers() {
        let input = PathBuf::from("test_data/input_large_numbers.txt");
        let items = read_items(&input).unwrap();
        assert_eq!(items.len(), 1);

        let result = generate_divisible_numbers(&input).unwrap();

        // Example: Test that it generates expected numbers for a large range
        // Assuming a specific input, adjust the expected output as needed
        let expected_numbers: Vec<u32> = (1..=100000).filter(|&n| n % 2 == 0 || n % 3 == 0).collect();
        assert_eq!(result[0].numbers, expected_numbers);
    }
}
