use std::io::Read;

const MAX_MEMORY: i32 = 1 << 16; // 65536

#[derive(Debug)]
pub struct Memory {
    container: Vec<u16>,
}

impl Memory {
    pub fn new() -> Memory {
        let container_size = MAX_MEMORY as usize;
        let container = std::iter::repeat(0)
            .take(container_size)
            .collect::<Vec<_>>();

        Memory { container }
    }
}

#[repr(u16)]
enum MemoryMappedRegisters {
    KeyboardStatus = 0xFE00, // KBSR
    KeyboardData = 0xFE02,   // KBDR
}

impl Memory {
    pub fn read(&mut self, address: u16) -> u16 {
        if address == MemoryMappedRegisters::KeyboardStatus as u16 {
            self.handle_keyboard();
        }

        self.container[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.container[address as usize] = value
    }
}

impl Memory {
    fn handle_keyboard(&mut self) {
        let mut buffer = [0 as u8; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write(MemoryMappedRegisters::KeyboardStatus as u16, 1 << 15);
            self.write(MemoryMappedRegisters::KeyboardData as u16, buffer[0] as u16);
        } else {
            self.write(MemoryMappedRegisters::KeyboardStatus as u16, 0)
        }
    }
}
