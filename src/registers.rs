#[path = "./condition_flags.rs"]
pub mod condition_flags;
use condition_flags::ConditionFlags;

#[derive(Debug)]
pub struct Registers {
    container: Vec<u16>,
}

#[repr(u16)]
enum RegisterCodes {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    ProgramCounter, // PC
    ConditionFlags, // COND
    Count,
}

impl Registers {
    pub fn new() -> Registers {
        let container_size = RegisterCodes::Count as usize;
        let container = std::iter::repeat(0)
            .take(container_size)
            .collect::<Vec<_>>();

        Registers { container }
    }
}

impl Registers {
    pub fn set_condition_flag(&mut self, condition_flag: ConditionFlags) {
        self.write(RegisterCodes::ConditionFlags, condition_flag as u16)
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.write(RegisterCodes::ProgramCounter, value)
    }

    pub fn increment_program_counter(&mut self) -> u16 {
        let new_value = self.read(RegisterCodes::ProgramCounter) + 1;
        self.write(RegisterCodes::ProgramCounter, new_value);

        new_value
    }

    pub fn read(&self, address: RegisterCodes) -> u16 {
        self.container[address as usize]
    }

    pub fn write(&mut self, address: RegisterCodes, value: u16) {
        self.container[address as usize] = value
    }
}
