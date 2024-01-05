use std::convert::TryFrom;
use std::env::args;
use std::error::Error;
use std::fs;

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

impl RawInstruction {
    fn from_char(c: char) -> Option<RawInstruction> {
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

struct InstructionWithPosition {
    instruction: RawInstruction,
    line_number: usize,
    character_column: usize,
}

impl InstructionWithPosition {
    pub fn new(instruction: RawInstruction, line_number: usize, column: usize) -> Self {
        Self {
            instruction: instruction,
            line_number: line_number,
            character_column: column,
        }
    }
}

fn parse_input_file(
    file_name: String,
) -> Result<Vec<InstructionWithPosition>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(file_name)?;

    let mut instructions: Vec<InstructionWithPosition> = Vec::new();
    let mut line_num: usize = 0;
    let mut col_num: usize = 0;
    for line in text.lines() {
        for c in line.chars() {
            match RawInstruction::from_char(c) {
                None => (),
                Some(instr) => {
                    let instr_w_pos = InstructionWithPosition::new(instr, line_num, col_num);
                    instructions.push(instr_w_pos);
                }
            }
            col_num += 1;
        }
        line_num += 1;
        col_num = 0;
    }

    Ok(instructions)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Usage: cargo run -- inputfilename")?;

    let instructions = parse_input_file(file_name)?;
    print!("\n");
    Ok(())
}
