use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, Program, no_input, no_output};

#[test]
fn test_change_relative_base() {
    let mut program = Program::new(vec![109, 19]);
    program.relative_base = 2000;
    program.run(no_input, no_output).unwrap();
    assert_eq!(program.relative_base, 2019);
}

#[test]
fn test_capabilities() {
    let code = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
    let mut program = Program::new(code.clone());
    let mut output = Vec::new();
    program.run(no_input, |i| { output.push(i); Ok(()) }).unwrap();
    assert_eq!(output, code);

    let mut program = Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
    let mut output = Vec::new();
    program.run(no_input, |i| { output.push(i); Ok(()) }).unwrap();
    assert_eq!(output.len(), 1);
    assert!(1_000_000_000_000_000 <= output[0]);
    assert!(output[0] <= 9_999_999_999_999_999);

    let mut program = Program::new(vec![104, 1125899906842624, 99]);
    let mut output = Vec::new();
    program.run(no_input, |i| { output.push(i); Ok(()) }).unwrap();
    assert_eq!(output, vec![1125899906842624]);
}

fn main() -> Res<()> {
    // Open the file
    let file = BufReader::new(File::open("inputs/day09.txt")?);

    // Read the program
    let mut program = Program::from_reader(file)?;

    // Part 1
    {
        // Run and get the output
        let mut output = 0;
        program.run(|| Ok(1), |i| { output = i; Ok(()) })?;

        println!("BOOST keycode: {}", output);
    }

    // Part 2
    {
        // Run and get the output
        let mut output = 0;
        program.run(|| Ok(2), |i| { output = i; Ok(()) })?;

        println!("Coordinates: {}", output);
    }

    Ok(())
}
