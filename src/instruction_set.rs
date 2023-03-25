#[repr(u16)]
#[derive(Debug)]
pub enum Opcodes {
    Branch = 0,           // BR
    Add,                  // ADD
    Load,                 // LD
    Store,                // ST
    JumpRegister,         // JSR
    BitwiseAnd,           // AND
    LoadRegister,         // LDR
    StoreRegister,        // STR
    Unused,               // RTI
    Not,                  // NOT
    LoadIndirect,         // LDI
    StoreIndirect,        // STI
    Jump,                 // JMP
    Reserved,             // RES (unused)
    LoadEffectiveAddress, // LEA
    ExecuteTrap,          // TRAP
}

impl Opcodes {
    pub fn from(value: u16) -> Option<Opcodes> {
        match value {
            0 => Some(Opcodes::Branch),
            1 => Some(Opcodes::Add),
            2 => Some(Opcodes::Load),
            3 => Some(Opcodes::Store),
            4 => Some(Opcodes::JumpRegister),
            5 => Some(Opcodes::BitwiseAnd),
            6 => Some(Opcodes::LoadRegister),
            7 => Some(Opcodes::StoreRegister),
            8 => Some(Opcodes::Unused),
            9 => Some(Opcodes::Not),
            10 => Some(Opcodes::LoadIndirect),
            11 => Some(Opcodes::StoreIndirect),
            12 => Some(Opcodes::Jump),
            13 => Some(Opcodes::Reserved),
            14 => Some(Opcodes::LoadEffectiveAddress),
            15 => Some(Opcodes::ExecuteTrap),
            _ => None,
        }
    }
}
