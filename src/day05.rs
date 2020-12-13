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
    let program = read_program(file)?;

    // Part 1
    {
        let mut program = program.clone();

        // Run it
        let mut output = 0;
        run_program(&mut program, || Ok(1), |i| { output = i; Ok(()) })?;

        // Print output
        println!("Output for diagnostic 1: {}", output);
    }

    // Part 2
    {
        let mut program = program.clone();

        // Run it
        let mut output = 0;
        run_program(&mut program, || Ok(5), |i| { output = i; Ok(()) })?;

        // Print output
        println!("Output for diagnostic 5: {}", output);
    }

    Ok(())
}
