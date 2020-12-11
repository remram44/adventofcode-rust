use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, read_program, run_program};

#[test]
fn test_exec() {
    let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert!(run_program(&mut program).is_ok());
    assert_eq!(&program, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

    let mut program = vec![1, 0, 0, 0, 99];
    assert!(run_program(&mut program).is_ok());
    assert_eq!(&program, &[2, 0, 0, 0, 99]);

    let mut program = vec![2, 3, 0, 3, 99];
    assert!(run_program(&mut program).is_ok());
    assert_eq!(&program, &[2, 3, 0, 6, 99]);

    let mut program = vec![2, 4, 4, 5, 99, 0];
    assert!(run_program(&mut program).is_ok());
    assert_eq!(&program, &[2, 4, 4, 5, 99, 9801]);

    let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    assert!(run_program(&mut program).is_ok());
    assert_eq!(&program, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

fn main() -> Res<()> {
    // Open the file
    let file = BufReader::new(File::open("inputs/day02.txt")?);

    // Read the program
    let program = read_program(file)?;

    // First part
    {
        let mut program = program.clone();

        // Set it up as required
        program[1] = 12;
        program[2] = 2;

        // Run it
        run_program(&mut program)?;

        // Print output
        println!("Output: {}", program[0]);
    }

    // Second part
    {
        // Try possible values
        for noun in 0..99 {
            for verb in 0..99 {
                let mut program = program.clone();
                program[1] = noun;
                program[2] = verb;
                match run_program(&mut program) {
                    Err(_) => {} // Not good
                    Ok(()) => {
                        if program[0] == 19690720 {
                            println!(
                                "noun = {}, verb = {} ; answer = {}",
                                noun, verb,
                                100 * noun + verb,
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
