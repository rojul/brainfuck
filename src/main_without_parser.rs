#[cfg(test)]
mod tests;
mod utils;

use std::io::{self, Read, Write};

fn main() {
    exec(
        &utils::file_from_args(),
        &mut io::stdin(),
        &mut io::stdout(),
    );
}

#[cfg(test)]
fn parse(bytes: &[u8]) -> Vec<u8> {
    bytes.to_vec()
}

fn exec<R: Read, W: Write>(bytes: &[u8], reader: &mut R, writer: &mut W) {
    let mut tape = vec![0u8; 1024];
    let mut pointer = tape.len() / 2;
    let mut pc = 0;
    let mut loop_starts = Vec::new();
    let mut loop_skips: usize = 0;
    while let Some(byte) = bytes.get(pc) {
        pc += 1;
        if loop_skips != 0 {
            match byte {
                b'[' => loop_skips += 1,
                b']' => loop_skips -= 1,
                _ => (),
            }
            continue;
        }
        match byte {
            b'+' => tape[pointer] = tape[pointer].wrapping_add(1),
            b'-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            b'<' => pointer -= 1,
            b'>' => pointer += 1,
            b'[' => {
                if tape[pointer] != 0 {
                    loop_starts.push(pc - 1);
                } else {
                    loop_skips += 1;
                }
            }
            b']' => pc = loop_starts.pop().expect("unexpected closing bracket"),
            b',' => tape[pointer] = utils::read(reader, writer),
            b'.' => writer.write_all(&[tape[pointer]]).unwrap(),
            _ => (),
        }
    }
    assert!(loop_starts.is_empty() && loop_skips == 0, "unclosed loop");
}
