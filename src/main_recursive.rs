#[cfg(test)]
mod tests;
mod utils;

use std::{
    io::{self, Read, Write},
    slice::Iter,
};

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
    Loop(Vec<Instruction>),
    Read,
    Write,
}

fn parse(bytes: &[u8]) -> Vec<Instruction> {
    parse_inner(&mut bytes.iter(), false)
}

fn parse_inner(bytes: &mut Iter<u8>, in_loop: bool) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    loop {
        let Some(&byte) = bytes.next() else {
            assert!(!in_loop, "unclosed loop");
            break;
        };
        let instruction = match byte {
            b'+' => Instruction::Plus,
            b'-' => Instruction::Minus,
            b'<' => Instruction::Left,
            b'>' => Instruction::Right,
            b'[' => Instruction::Loop(parse_inner(bytes, true)),
            b']' => {
                assert!(in_loop, "unexpected closing bracket");
                break;
            }
            b',' => Instruction::Read,
            b'.' => Instruction::Write,
            _ => continue,
        };
        instructions.push(instruction);
    }
    instructions
}

fn exec<R: Read, W: Write>(instructions: &[Instruction], reader: &mut R, writer: &mut W) {
    let mut tape = vec![0u8; 1024];
    let mut pointer = tape.len() / 2;
    exec_inner(instructions, reader, writer, &mut tape, &mut pointer);
}

fn exec_inner<R: Read, W: Write>(
    instructions: &[Instruction],
    reader: &mut R,
    writer: &mut W,
    tape: &mut Vec<u8>,
    pointer: &mut usize,
) {
    for instruction in instructions {
        match instruction {
            Instruction::Plus => {
                tape[*pointer] = tape[*pointer].wrapping_add(1);
            }
            Instruction::Minus => {
                tape[*pointer] = tape[*pointer].wrapping_sub(1);
            }
            Instruction::Left => {
                *pointer -= 1;
            }
            Instruction::Right => {
                *pointer += 1;
            }
            Instruction::Loop(loop_instructions) => {
                while tape[*pointer] != 0 {
                    exec_inner(loop_instructions, reader, writer, tape, pointer)
                }
            }
            Instruction::Read => {
                tape[*pointer] = utils::read(reader, writer);
            }
            Instruction::Write => {
                writer.write_all(&[tape[*pointer]]).unwrap();
            }
        }
    }
}
