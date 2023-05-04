use std::{
    env, fs,
    io::{Read, Write},
    slice,
};

pub fn file_from_args() -> Vec<u8> {
    let file = env::args().nth(1).expect("should provide brainfuck file as argument");
    fs::read(file).expect("file could not be read")
}

pub fn read<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> u8 {
    writer.flush().unwrap();
    let mut byte = 0;
    loop {
        reader.read_exact(slice::from_mut(&mut byte)).unwrap();
        if byte != b'\n' {
            break;
        }
    }
    byte
}
