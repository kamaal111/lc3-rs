mod instruction_set;
mod memory;
mod registers;
use instruction_set::Opcodes;
use memory::Memory;
use registers::Registers;

fn main() {
    // Load arguments

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