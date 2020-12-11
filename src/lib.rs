use std::io::Read;

pub type Res<O> = Result<O, Box<dyn std::error::Error>>;

pub fn read_program<R: Read>(mut file: R) -> Res<Vec<i32>> {
    let mut program = Vec::new();

    let mut position = 0;
    let mut negative = false;
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
        if byte == b'-' {
            if number != 0 || negative {
                return Err(format!("Unexpected - sign at {}", position).into());
            }
            negative = true;
        } else if b'0' <= byte && byte <= b'9' {
            number = number * 10 + (byte - b'0') as i32;
        } else if byte == b',' || byte == b'\n' {
            program.push(if negative { -number } else { number });
            number = 0;
            negative = false;
            if byte == b'\n' {
                break;
            }
        } else {
            return Err(
                format!("Invalid character at {}: 0x{:x}", position, byte).into(),
            );
        }
        position += 1;
    }

    Ok(program)
}

fn read(program: &Vec<i32>, pos: i32) -> Res<i32> {
    if pos < 0 {
        Err("Read negative offset".into())
    } else if pos as usize >= program.len() {
        Err("Read exceeds memory size".into())
    } else {
        Ok(program[pos as usize])
    }
}

fn write(program: &mut Vec<i32>, pos: i32, value: i32) -> Res<()> {
    if pos < 0 {
        Err("Write negative offset".into())
    } else if pos as usize >= program.len() {
        Err("Write exceeds memory size".into())
    } else {
        program[pos as usize] = value;
        Ok(())
    }
}

pub fn step_program<I, O>(
    program: &mut Vec<i32>,
    counter: &mut usize,
    mut input: I,
    mut output: O,
) -> Res<bool>
where
    I: FnMut() -> Res<i32>,
    O: FnMut(i32) -> Res<()>,
{
    if *counter >= program.len() {
        Ok(false)
    } else {
        let instr = read(program, *counter as i32)?;
        if instr == 99 {
            // Halt
            return Ok(false);
        } else if instr == 1 {
            let op1 = read(program, read(program, *counter as i32 + 1)?)?;
            let op2 = read(program, read(program, *counter as i32 + 2)?)?;
            let target = read(program, *counter as i32 + 3)?;
            write(program, target, op1 + op2)?;
            *counter += 4;
        } else if instr == 2 {
            let op1 = read(program, read(program, *counter as i32 + 1)?)?;
            let op2 = read(program, read(program, *counter as i32 + 2)?)?;
            let target = read(program, *counter as i32 + 3)?;
            write(program, target, op1 * op2)?;
            *counter += 4;
        } else if instr == 3 {
            let op = read(program, *counter as i32 + 1)?;
            write(program, op, input()?)?;
        } else if instr == 4 {
            let op = read(program, *counter as i32 + 1)?;
            output(op)?;
        } else {
            return Err(format!("Unknown instruction {} at position {}", instr, counter).into());
        }
        Ok(true)
    }
}

pub fn run_program<I, O>(
    program: &mut Vec<i32>,
    mut input: I,
    mut output: O,
) -> Res<()>
where
    I: FnMut() -> Res<i32>,
    O: FnMut(i32) -> Res<()>,
{
    let mut counter = 0;
    loop {
        if !step_program(program, &mut counter, &mut input, &mut output)? {
            return Ok(());
        }
    }
}

pub fn no_input() -> Res<i32> {
    Err("No input available".into())
}

pub fn no_output(_: i32) -> Res<()> {
    Err("No output possible".into())
}
