mod memory;
mod registers;
use memory::Memory;
use registers::Registers;

fn main() {
    // Load arguments

    // Setup
    let memory = Memory::new();
    let mut registers = Registers::new();

    // since exactly one condition flag should be set at any given time, set the Z flag
    registers.set_condition_flag(registers::condition_flags::ConditionFlags::Zero);
    println!("{:?}", registers);
}
