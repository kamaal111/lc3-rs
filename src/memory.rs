const MAX_MEMORY: i32 = 1 << 16; // 65536

#[derive(Debug)]
pub struct Memory {
    container: Vec<u16>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            container: Vec::with_capacity(MAX_MEMORY as usize),
        }
    }
}
