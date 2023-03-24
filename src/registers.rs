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
    ProgramCounter,
    ConditionFlags,
    Count,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            container: Vec::with_capacity(RegisterCodes::Count as usize),
        }
    }
}
