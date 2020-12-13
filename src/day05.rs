use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, read_program, run_program};

#[test]
fn test_compare() {
    // Using position mode, consider whether the input is equal to 8; output 1
    // (if it is) or 0 (if it is not).
    let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut output = 42;
    run_program(&mut program, || Ok(7), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);
    run_program(&mut program, || Ok(8), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 1);
    run_program(&mut program, || Ok(9), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);

    // Using position mode, consider whether the input is less than 8; output 1
    // (if it is) or 0 (if it is not).
    let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut output = 42;
    run_program(&mut program, || Ok(7), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 1);
    run_program(&mut program, || Ok(8), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);
    run_program(&mut program, || Ok(9), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);

    // Using immediate mode, consider whether the input is equal to 8; output 1
    // (if it is) or 0 (if it is not).
    let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut output = 42;
    run_program(&mut program, || Ok(7), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);
    run_program(&mut program, || Ok(8), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 1);
    run_program(&mut program, || Ok(9), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);

    // Using immediate mode, consider whether the input is less than 8; output 1
    // (if it is) or 0 (if it is not).
    let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut output = 42;
    run_program(&mut program, || Ok(7), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 1);
    run_program(&mut program, || Ok(8), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);
    run_program(&mut program, || Ok(9), |i| { output = i; Ok(()) }).unwrap();
    assert_eq!(output, 0);
}

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
