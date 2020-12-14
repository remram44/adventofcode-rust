use std::fs::File;
use std::io::BufReader;

use adventofcode2019::{Res, Program, no_input, no_output};

#[test]
fn test_exec() {
    let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    program.run(no_input, no_output).unwrap();
    assert_eq!(&program.memory, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

    let mut program = Program::new(vec![1, 0, 0, 0, 99]);
    program.run(no_input, no_output).unwrap();
    assert_eq!(&program.memory, &[2, 0, 0, 0, 99]);

    let mut program = Program::new(vec![2, 3, 0, 3, 99]);
    program.run(no_input, no_output).unwrap();
    assert_eq!(&program.memory, &[2, 3, 0, 6, 99]);

    let mut program = Program::new(vec![2, 4, 4, 5, 99, 0]);
    program.run(no_input, no_output).unwrap();
    assert_eq!(&program.memory, &[2, 4, 4, 5, 99, 9801]);

    let mut program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
    program.run(no_input, no_output).unwrap();
    assert_eq!(&program.memory, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

fn main() -> Res<()> {
    // Open the file
    let file = BufReader::new(File::open("inputs/day02.txt")?);

    // Read the program
    let program = Program::from_reader(file)?;

    // First part
    {
        let mut program = program.clone();

        // Set it up as required
        program.memory[1] = 12;
        program.memory[2] = 2;

        // Run it
        program.run(no_input, no_output)?;

        // Print output
        println!("Output: {}", program.memory[0]);
    }

    // Second part
    {
        // Try possible values
        for noun in 0..99 {
            for verb in 0..99 {
                let mut program = program.clone();
                program.memory[1] = noun;
                program.memory[2] = verb;
                match program.run(no_input, no_output) {
                    Err(_) => {} // Not good
                    Ok(()) => {
                        if program.memory[0] == 19690720 {
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
