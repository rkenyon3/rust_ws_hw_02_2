use std::env::args;
use std::fs;
use std::error::Error;
use std::convert::TryFrom;

enum RawInstruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
    EndLoop,
}

impl RawInstruction{
    fn from_char(c: char) -> Option<RawInstruction>{
        match c {
            '>' => Some(RawInstruction::MoveRight),
            '<' => Some(RawInstruction::MoveLeft),
            '+' => Some(RawInstruction::Increment),
            '-' => Some(RawInstruction::Decrement),
            '.' => Some(RawInstruction::Input),
            ',' => Some(RawInstruction::Output),
            '[' => Some(RawInstruction::BeginLoop),
            ']' => Some(RawInstruction::EndLoop),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Usage: cargo run -- inputfilename")?;

    let text = fs::read_to_string(file_name)?;
    
    let mut program_chars: Vec<char> = Vec::new();
    for c in text.chars(){
        if "><+_.,[]".contains(c){
            program_chars.push(c)
        }
    }

    // print the bf program to the screen
    for c in program_chars.iter() {
        print!("{c}");
    }
    print!("\n");
    Ok(())
}
