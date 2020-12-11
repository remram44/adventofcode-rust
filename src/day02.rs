use std::fs::File;
use std::io::{BufReader, Read};

fn read_program<R: Read>(mut file: R) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut program = Vec::new();

    let mut number: i32 = 0;
    loop {
        let byte = {
            let mut buf = [0u8];
            match file.read(&mut buf)? {
                0 => b'\n',
                1 => buf[0],
                _ => panic!("Invalid return from read()"),
            }
        };
        if b'0' <= byte && byte <= b'9' {
            number = number * 10 + (byte - b'0') as i32;
        } else if byte == b',' || byte == b'\n' {
            program.push(number);
            number = 0;
            if byte == b'\n' {
                break;
            }
        } else {
            return Err(
                format!("Invalid character in input: {:?}", byte).into(),
            );
        }
    }

    Ok(program)
}

fn read(program: &Vec<i32>, pos: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if pos < 0 {
        Err("Read negative offset".into())
    } else if pos as usize >= program.len() {
        Err("Read exceeds memory size".into())
    } else {
        Ok(program[pos as usize])
    }
}

fn write(program: &mut Vec<i32>, pos: i32, value: i32) -> Result<(), Box<dyn std::error::Error>> {
    if pos < 0 {
        Err("Write negative offset".into())
    } else if pos as usize >= program.len() {
        Err("Write exceeds memory size".into())
    } else {
        program[pos as usize] = value;
        Ok(())
    }
}

fn step_program(program: &mut Vec<i32>, counter: &mut usize) -> Result<bool, Box<dyn std::error::Error>> {
    if *counter >= program.len() {
        Ok(false)
    } else {
        let instr = read(program, *counter as i32)?;
        if instr == 99 {
            // Halt
            Ok(false)
        } else if instr == 1 {
            let op1 = read(program, read(program, *counter as i32 + 1)?)?;
            let op2 = read(program, read(program, *counter as i32 + 2)?)?;
            let target = read(program, *counter as i32 + 3)?;
            write(program, target, op1 + op2)?;
            *counter += 4;
            Ok(true)
        } else if instr == 2 {
            let op1 = read(program, read(program, *counter as i32 + 1)?)?;
            let op2 = read(program, read(program, *counter as i32 + 2)?)?;
            let target = read(program, *counter as i32 + 3)?;
            write(program, target, op1 * op2)?;
            *counter += 4;
            Ok(true)
        } else {
            Err(format!("Unknown instruction {} at position {}", instr, counter).into())
        }
    }
}

fn run_program(program: &mut Vec<i32>) -> Result<(), Box<dyn std::error::Error>> {
    let mut counter = 0;
    loop {
        if !step_program(program, &mut counter)? {
            return Ok(());
        }
    }
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
