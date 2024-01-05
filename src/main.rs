use std::env::args;
use std::fs;
use std::error::Error;

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
