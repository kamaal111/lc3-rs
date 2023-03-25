mod instruction_set;
mod memory;
mod registers;

use clap::Parser;
use instruction_set::Opcodes;
use memory::Memory;
use registers::Registers;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_path: String,
}

fn main() {
    // Load arguments
    let args = Args::parse();
    let file_path = args.file_path;

    let program = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", program);

    // Setup
    let mut memory = Memory::new();
    let mut registers = Registers::new();

    // since exactly one condition flag should be set at any given time, set the Z flag
    registers.set_condition_flag(registers::condition_flags::ConditionFlags::Zero);

    // set the Program Counter to starting position
    // 0x3000 (12288) is the default
    let program_counter_start = 0x3000 as u16;
    registers.set_program_counter(program_counter_start);

    let mut running = 1;
    while running > 0 {
        // Fetch opcode
        let new_program_counter_value = registers.increment_program_counter();
        let instruction = memory.read(new_program_counter_value);
        let opcode = instruction >> 12;
        let opcode = Opcodes::from(opcode);
        println!("{:?}", opcode);
        running -= 1;
        break;
    }

    println!("{:?} {}", registers, running);
}
