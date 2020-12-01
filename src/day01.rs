use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open("inputs/day01.txt")?;

    // Wrap it in a buffered reader, so we can read lines
    let file = BufReader::new(file);

    // Build an array of integers from the lines of the file
    let mut numbers = Vec::new();
    for line in file.lines() {
        let line = line?;

        // Remove line delimiter (\n)
        let trimmed = line.trim_end();

        // Parse it as an integer
        let num: u32 = trimmed.parse()?;
        numbers.push(num);
    }

    // Find desired pair
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!(
                    "{} * {} = {}",
                    numbers[i],
                    numbers[j],
                    numbers[i] * numbers[j],
                );
            }
        }
    }

    // Find desired triple
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!(
                        "{} * {} * {} = {}",
                        numbers[i],
                        numbers[j],
                        numbers[k],
                        numbers[i] * numbers[j] * numbers[k],
                    );
                }
            }
        }
    }

    Ok(())
}
