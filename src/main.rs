mod memory;
mod registers;
use memory::Memory;
use registers::Registers;

fn main() {
    let memory = Memory::new();
    let registers = Registers::new();
    println!("{:?} {:?}", memory, registers);
}
