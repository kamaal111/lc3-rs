#[path = "./instruction_set.rs"]
mod instruction_set;
#[path = "./memory.rs"]
mod memory;
#[path = "./registers.rs"]
mod registers;

use instruction_set::Opcodes;
use memory::Memory;
use registers::Registers;

#[derive(Debug)]
pub struct VirtualMachine {
    memory: Memory,
    registers: Registers,
    program: String,
}

impl VirtualMachine {
    pub fn run(program: String) {
        let mut virtual_machine = VirtualMachine::initial_setup(program);

        let mut running = 1;
        while running > 0 {
            let new_program_counter_value = virtual_machine.registers.increment_program_counter();
            let instruction = virtual_machine.memory.read(new_program_counter_value);
            let opcode = instruction >> 12;

            let opcode = match Opcodes::from(opcode) {
                Some(value) => value,
                None => {
                    println!("bad code");
                    continue;
                }
            };

            virtual_machine.perform_instruction(opcode, instruction);

            running -= 1;
            break;
        }

        println!("running {}", running);
    }
}

impl VirtualMachine {
    fn perform_instruction(&mut self, opcode: Opcodes, instruction: u16) {
        match opcode {
            Opcodes::Branch => todo!(),
            Opcodes::Add => self.registers.add(instruction),
            Opcodes::Load => todo!(),
            Opcodes::Store => todo!(),
            Opcodes::JumpRegister => todo!(),
            Opcodes::BitwiseAnd => todo!(),
            Opcodes::LoadRegister => todo!(),
            Opcodes::StoreRegister => todo!(),
            Opcodes::Unused => todo!(),
            Opcodes::Not => todo!(),
            Opcodes::LoadIndirect => todo!(),
            Opcodes::StoreIndirect => todo!(),
            Opcodes::Jump => todo!(),
            Opcodes::Reserved => todo!(),
            Opcodes::LoadEffectiveAddress => todo!(),
            Opcodes::ExecuteTrap => todo!(),
        }
    }
}

impl VirtualMachine {
    fn initial_setup(program: String) -> VirtualMachine {
        let memory = Memory::new();
        let registers = Registers::new();

        let mut virtual_machine = VirtualMachine {
            memory,
            registers,
            program,
        };

        // since exactly one condition flag should be set at any given time, set the Zero flag
        virtual_machine
            .registers
            .set_condition_flag(registers::condition_flags::ConditionFlags::Zero);

        // set the Program Counter to starting position
        // 0x3000 (12288) is the default
        let program_counter_start = 0x3000 as u16;
        virtual_machine
            .registers
            .set_program_counter(program_counter_start);

        virtual_machine
    }
}
