#[repr(u16)]
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
