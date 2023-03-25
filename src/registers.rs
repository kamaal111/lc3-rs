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

    pub fn set_condition_flag(&mut self, condition_flag: ConditionFlags) {
        self.container[RegisterCodes::ConditionFlags as usize] = condition_flag as u16
    }
}
