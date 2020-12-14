use std::io::Read;

pub type Res<O> = Result<O, Box<dyn std::error::Error>>;

fn read_program<R: Read>(mut file: R) -> Res<Vec<i32>> {
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

#[derive(Clone)]
pub struct Program {
    pub memory: Vec<i32>,
    pub counter: usize,
}

impl Program {
    pub fn new(memory: Vec<i32>) -> Program {
        Program {
            memory,
            counter: 0,
        }
    }

    pub fn from_reader<R: Read>(file: R) -> Res<Program> {
        let memory = read_program(file)?;
        Ok(Program::new(memory))
    }

    fn read(&self, pos: Parameter) -> Res<i32> {
        match pos {
            Parameter::Position(addr) => {
                if addr < 0 {
                    Err("Read negative offset".into())
                } else if addr as usize >= self.memory.len() {
                    Err("Read exceeds memory size".into())
                } else {
                    Ok(self.memory[addr as usize])
                }
            }
            Parameter::Immediate(v) => Ok(v),
        }
    }

    fn write(&mut self, pos: Parameter, value: i32) -> Res<()> {
        match pos {
            Parameter::Position(addr) => {
                if addr < 0 {
                    Err("Write negative offset".into())
                } else if addr as usize >= self.memory.len() {
                    Err("Write exceeds memory size".into())
                } else {
                    self.memory[addr as usize] = value;
                    Ok(())
                }
            }
            Parameter::Immediate(_) => Err("Can't write on immediate value".into()),
        }
    }

    fn get_parameter(
        &mut self,
        decoder: &mut ParameterDecoder,
    ) -> Res<Parameter> {
        let param = decoder.decode_parameter(self.memory[self.counter])?;
        self.counter += 1;
        Ok(param)
    }

    fn read_parameter(
        &mut self,
        decoder: &mut ParameterDecoder,
    ) -> Res<i32> {
        let op = self.get_parameter(decoder)?;
        self.read(op)
    }

    pub fn step<I, O>(
        &mut self,
        mut input: I,
        mut output: O,
    ) -> Res<bool>
    where
        I: FnMut() -> Res<i32>,
        O: FnMut(i32) -> Res<()>,
    {
        if self.counter >= self.memory.len() {
            Ok(false)
        } else {
            let instr = self.read(Parameter::Position(self.counter as i32))?;
            self.counter += 1;
            let (instr, mut decoder) = decode_instruction(instr)?;
            if instr == 99 {
                // Halt
                return Ok(false);
            } else if instr == 1 {
                let op1 = self.read_parameter(&mut decoder)?;
                let op2 = self.read_parameter(&mut decoder)?;
                let target = self.get_parameter(&mut decoder)?;
                self.write(target, op1 + op2)?;
            } else if instr == 2 {
                let op1 = self.read_parameter(&mut decoder)?;
                let op2 = self.read_parameter(&mut decoder)?;
                let target = self.get_parameter(&mut decoder)?;
                self.write(target, op1 * op2)?;
            } else if instr == 3 {
                let target = self.get_parameter(&mut decoder)?;
                self.write(target, input()?)?;
            } else if instr == 4 {
                let op = self.read_parameter(&mut decoder)?;
                output(op)?;
            } else if instr == 5 {
                let op1 = self.read_parameter(&mut decoder)?;
                let op2 = self.read_parameter(&mut decoder)?;
                if op1 != 0 {
                    if op2 < 0 {
                        return Err(format!("Attempt to jump to {} at position {}", op2, self.counter).into());
                    }
                    self.counter = op2 as usize;
                }
            } else if instr == 6 {
                let op1 = self.read_parameter(&mut decoder)?;
                let op2 = self.read_parameter(&mut decoder)?;
                if op1 == 0 {
                    if op2 < 0 {
                        return Err(format!("Attempt to jump to {} at position {}", op2, self.counter).into());
                    }
                    self.counter = op2 as usize;
                }
            } else if instr == 7 {
                let op1 = self.read_parameter(&mut decoder)?;
                let op2 = self.read_parameter(&mut decoder)?;
                let target = self.get_parameter(&mut decoder)?;
                self.write(target, if op1 < op2 { 1 } else { 0 })?;
            } else if instr == 8 {
                let op1 = self.read_parameter(&mut decoder)?;
                let op2 = self.read_parameter(&mut decoder)?;
                let target = self.get_parameter(&mut decoder)?;
                self.write(target, if op1 == op2 { 1 } else { 0 })?;
            } else {
                return Err(format!("Unknown instruction {} at position {}", instr, self.counter).into());
            }
            Ok(true)
        }
    }

    pub fn run<I, O>(&mut self, mut input: I, mut output: O) -> Res<()>
    where
        I: FnMut() -> Res<i32>,
        O: FnMut(i32) -> Res<()>,
    {
        loop {
            if !self.step(&mut input, &mut output)? {
                return Ok(());
            }
        }
    }
}

pub fn no_input() -> Res<i32> {
    Err("No input available".into())
}

pub fn no_output(_: i32) -> Res<()> {
    Err("No output possible".into())
}
