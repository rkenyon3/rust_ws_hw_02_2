use std::env::args;
use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
enum RawInstruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    ConditionalJumpForward,
    ConditionalJumpBackward
}

impl RawInstruction {
    fn from_char(c: char) -> Option<RawInstruction> {
        match c {
            '>' => Some(RawInstruction::MoveRight),
            '<' => Some(RawInstruction::MoveLeft),
            '+' => Some(RawInstruction::Increment),
            '-' => Some(RawInstruction::Decrement),
            ',' => Some(RawInstruction::Input),
            '.' => Some(RawInstruction::Output),
            '[' => Some(RawInstruction::ConditionalJumpForward),
            ']' => Some(RawInstruction::ConditionalJumpBackward),
            _ => None,
        }
    }
}





#[derive(Debug)]
struct InstructionWithPosition {
    instruction: RawInstruction,
    line_number: usize,
    character_column: usize,
}

impl InstructionWithPosition {
    pub fn new(instruction: RawInstruction, line_number: usize, character_column: usize) -> Self {
        Self {
            instruction,
            line_number,
            character_column,
        }
    }
}

impl Display for InstructionWithPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instr_str_rep = match self.instruction {
            RawInstruction::MoveLeft => "Move Left",            RawInstruction::MoveRight => "Move Right",            RawInstruction::ConditionalJumpForward => "Conditional Jump Forward",            RawInstruction::ConditionalJumpBackward => "Conditional Jump Backward",            RawInstruction::Increment => "Increment",
            RawInstruction::Decrement => "Decrement",
            RawInstruction::Input => "Input",
            RawInstruction::Output => "Output",
        };

        write!(
            f,
            "{}:{}\t{}",
            self.line_number + 1,
            self.character_column + 1,
            instr_str_rep
        )
    }
}

fn parse_input_file(
    file_name: &String,
) -> Result<Vec<InstructionWithPosition>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(file_name)?;

    let mut instructions: Vec<InstructionWithPosition> = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        for (char_num, character) in line.chars().enumerate() {
            match RawInstruction::from_char(character) {
                None => (),
                Some(instr) => {
                    let instr_w_pos = InstructionWithPosition::new(instr, line_num, char_num);
                    instructions.push(instr_w_pos);
                }
            }
        }
    }

    Ok(instructions)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Usage: cargo run -- inputfilename")?;

    let instructions = parse_input_file(&file_name)?;
    for instr in instructions.iter() {
        println!("{}:{}", &file_name, instr)
    }
    Ok(())
}
