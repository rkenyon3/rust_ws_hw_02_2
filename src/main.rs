use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;

fn is_valid_character(c: &char) -> bool {
    match c {
        '>' => true,
        '<' => true,
        '+' => true,
        '-' => true,
        '.' => true,
        ',' => true,
        '[' => true,
        ']' => true,
        _ => false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Usage: cargo run -- inputfilename")?;

    let mut program_chars: Vec<char> = Vec::new();

    let file = File::open(file_name).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    // buffer to read file into
    let mut buffer: [u8; 100] = [0; 100];

    // read the file contents in chunks, and store program characters in program_chars
    let mut byte_count = reader.read(&mut buffer[..])?;
    while byte_count > 0 {
        let mut i = 0;
        while i < byte_count {
            let b = buffer[i] as u32;
            let c = char::from_u32(b).ok_or("failed to parse character")?;
            if is_valid_character(&c) {
                program_chars.push(c);
            }
            i += 1;
        }
        byte_count = reader.read(&mut buffer[..])?;
    }

    // print the bf program to the screen
    for c in program_chars.iter() {
        print!("{c}");
    }
    print!("\n");
    Ok(())
}
