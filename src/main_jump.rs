#[cfg(test)]
mod tests;
mod utils;

use std::io::{self, Read, Write};

fn main() {
    let instructions = parse(&utils::file_from_args());
    exec(&instructions, &mut io::stdin(), &mut io::stdout());
}

#[derive(Debug)]
enum Instruction {
    Plus,
    Minus,
    Left,
    Right,
    Loop { end: usize },
    LoopEnd { start: usize },
    Read,
    Write,
}

fn parse(bytes: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    let mut loop_starts = Vec::new();
    for &byte in bytes {
        let instruction = match byte {
            b'+' => Instruction::Plus,
            b'-' => Instruction::Minus,
            b'<' => Instruction::Left,
            b'>' => Instruction::Right,
            b'[' => {
                loop_starts.push(instructions.len());
                Instruction::Loop { end: usize::MAX }
            }
            b']' => {
                let start = loop_starts.pop().expect("unexpected closing bracket");
                let len = instructions.len();
                let Instruction::Loop { end } = &mut instructions[start] else {
                    unreachable!();
                };
                *end = len + 1;
                Instruction::LoopEnd { start }
            }
            b',' => Instruction::Read,
            b'.' => Instruction::Write,
            _ => continue,
        };
        instructions.push(instruction);
    }
    assert!(loop_starts.is_empty(), "unclosed loop");
    instructions
}

fn exec<R: Read, W: Write>(instructions: &[Instruction], reader: &mut R, writer: &mut W) {
    let mut tape = vec![0u8; 1024];
    let mut pointer = tape.len() / 2;
    let mut pc = 0;
    while let Some(instruction) = instructions.get(pc) {
        match instruction {
            Instruction::Plus => {
                tape[pointer] = tape[pointer].wrapping_add(1);
            }
            Instruction::Minus => {
                tape[pointer] = tape[pointer].wrapping_sub(1);
            }
            Instruction::Left => {
                pointer -= 1;
            }
            Instruction::Right => {
                pointer += 1;
            }
            Instruction::Loop { end } => {
                if tape[pointer] == 0 {
                    pc = *end;
                    continue;
                }
            }
            Instruction::LoopEnd { start } => {
                pc = *start;
                continue;
            }
            Instruction::Read => {
                tape[pointer] = utils::read(reader, writer);
            }
            Instruction::Write => {
                writer.write_all(&[tape[pointer]]).unwrap();
            }
        }
        pc += 1;
    }
}
