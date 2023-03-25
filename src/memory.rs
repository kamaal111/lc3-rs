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
