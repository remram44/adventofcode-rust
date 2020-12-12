use std::io::Read;

pub type Res<O> = Result<O, Box<dyn std::error::Error>>;

pub fn read_program<R: Read>(mut file: R) -> Res<Vec<i32>> {
    let mut memory = Vec::new();

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
            memory.push(if negative { -number } else { number });
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

    Ok(memory)
}

fn read(memory: &Vec<i32>, pos: Parameter) -> Res<i32> {
    match pos {
        Parameter::Position(addr) => {
            if addr < 0 {
                Err("Read negative offset".into())
            } else if addr as usize >= memory.len() {
                Err("Read exceeds memory size".into())
            } else {
                Ok(memory[addr as usize])
            }
        }
        Parameter::Immediate(v) => Ok(v),
    }
}

fn write(memory: &mut Vec<i32>, pos: Parameter, value: i32) -> Res<()> {
    match pos {
        Parameter::Position(addr) => {
            if addr < 0 {
                Err("Write negative offset".into())
            } else if addr as usize >= memory.len() {
                Err("Write exceeds memory size".into())
            } else {
                memory[addr as usize] = value;
                Ok(())
            }
        }
        Parameter::Immediate(_) => Err("Can't write on immediate value".into()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Parameter {
    Position(i32),
    Immediate(i32),
}

struct ParameterDecoder(i32);

impl ParameterDecoder {
    fn decode_parameter(&mut self, value: i32) -> Res<Parameter> {
        let code = self.0 % 10;
        self.0 /= 10;
        match code {
            0 => Ok(Parameter::Position(value)),
            1 => Ok(Parameter::Immediate(value)),
            _ => Err(format!("Invalid parameter mode {}", code).into()),
        }
    }
}

fn decode_instruction(code: i32) -> Res<(i32, ParameterDecoder)> {
    if code <= 0 {
        Err(format!("Invalid opcode {}", code).into())
    } else {
        let instr = code % 100;
        let param_codes = code / 100;
        Ok((instr, ParameterDecoder(param_codes)))
    }
}

#[test]
fn test_decode() {
    let (instr, mut decoder) = decode_instruction(1002).unwrap();
    assert_eq!(instr, 2);
    assert_eq!(decoder.decode_parameter(421).unwrap(), Parameter::Position(421));
    assert_eq!(decoder.decode_parameter(422).unwrap(), Parameter::Immediate(422));
    assert_eq!(decoder.decode_parameter(423).unwrap(), Parameter::Position(423));
}

fn get_parameter(
    decoder: &mut ParameterDecoder,
    memory: &Vec<i32>,
    pos: &mut usize,
) -> Res<Parameter> {
    let param = decoder.decode_parameter(memory[*pos])?;
    *pos += 1;
    Ok(param)
}

fn read_parameter(
    decoder: &mut ParameterDecoder,
    memory: &Vec<i32>,
    pos: &mut usize,
) -> Res<i32> {
    let op = get_parameter(decoder, memory, pos)?;
    read(memory, op)
}

pub fn step_program<I, O>(
    memory: &mut Vec<i32>,
    counter: &mut usize,
    mut input: I,
    mut output: O,
) -> Res<bool>
where
    I: FnMut() -> Res<i32>,
    O: FnMut(i32) -> Res<()>,
{
    if *counter >= memory.len() {
        Ok(false)
    } else {
        let instr = read(memory, Parameter::Position(*counter as i32))?;
        *counter += 1;
        let (instr, mut decoder) = decode_instruction(instr)?;
        if instr == 99 {
            // Halt
            return Ok(false);
        } else if instr == 1 {
            let op1 = read_parameter(&mut decoder, &memory, counter)?;
            let op2 = read_parameter(&mut decoder, &memory, counter)?;
            let target = get_parameter(&mut decoder, &memory, counter)?;
            write(memory, target, op1 + op2)?;
        } else if instr == 2 {
            let op1 = read_parameter(&mut decoder, &memory, counter)?;
            let op2 = read_parameter(&mut decoder, &memory, counter)?;
            let target = get_parameter(&mut decoder, &memory, counter)?;
            write(memory, target, op1 * op2)?;
        } else if instr == 3 {
            let target = get_parameter(&mut decoder, &memory, counter)?;
            write(memory, target, input()?)?;
        } else if instr == 4 {
            let op = read_parameter(&mut decoder, &memory, counter)?;
            output(op)?;
        } else {
            return Err(format!("Unknown instruction {} at position {}", instr, counter).into());
        }
        Ok(true)
    }
}

pub fn run_program<I, O>(
    memory: &mut Vec<i32>,
    mut input: I,
    mut output: O,
) -> Res<()>
where
    I: FnMut() -> Res<i32>,
    O: FnMut(i32) -> Res<()>,
{
    let mut counter = 0;
    loop {
        if !step_program(memory, &mut counter, &mut input, &mut output)? {
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
