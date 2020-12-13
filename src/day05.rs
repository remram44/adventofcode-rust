use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, read_program, run_program};

fn main() -> Res<()> {
    // Open the file
    let file = BufReader::new(File::open("inputs/day05.txt")?);

    // Read the program
    let mut program = read_program(file)?;

    // Input: "1, ID of the air condition unit"
    let input = || Ok(1);

    // Output: 0 ok, 1 indicates error in self-test
    let mut last_output = 0;
    let output = |i: i32| -> Res<()> {
        if last_output != 0 {
            Err(format!("Error from self-test (output {})", last_output).into())
        } else {
            last_output = i;
            Ok(())
        }
    };

    // Run it
    run_program(&mut program, input, output)?;

    // Print output
    println!("Output: {}", last_output);

    Ok(())
}
