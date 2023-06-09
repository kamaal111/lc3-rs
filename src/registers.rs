#[path = "./condition_flags.rs"]
pub mod condition_flags;
use std::io::{self, Write};

use condition_flags::ConditionFlags;

use super::memory::Memory;

#[derive(Debug)]
pub struct Registers {
    container: Vec<u16>,
}

#[repr(u16)]
#[derive(Clone)]
pub enum RegisterCodes {
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

impl RegisterCodes {
    pub fn from(value: u16) -> Option<RegisterCodes> {
        match value {
            0 => Some(RegisterCodes::R0),
            1 => Some(RegisterCodes::R1),
            2 => Some(RegisterCodes::R2),
            3 => Some(RegisterCodes::R3),
            4 => Some(RegisterCodes::R4),
            5 => Some(RegisterCodes::R5),
            6 => Some(RegisterCodes::R6),
            7 => Some(RegisterCodes::R7),
            8 => Some(RegisterCodes::ProgramCounter),
            9 => Some(RegisterCodes::ConditionFlags),
            10 => Some(RegisterCodes::Count),
            _ => None,
        }
    }
}

#[repr(u16)]
enum TrapCodes {
    GetCharacter = 0x20,     // GETC
    OutputCharacter,         // OUT
    OutputWord,              // PUTS
    EchoCharacterToTerminal, // IN
    OutputByteString,        // PUTSP
    Halt,                    // HALT
}

impl TrapCodes {
    fn from(value: u16) -> Option<Self> {
        match value {
            0x20 => Some(TrapCodes::GetCharacter),
            0x21 => Some(TrapCodes::OutputCharacter),
            0x22 => Some(TrapCodes::OutputWord),
            0x23 => Some(TrapCodes::EchoCharacterToTerminal),
            0x24 => Some(TrapCodes::OutputByteString),
            0x25 => Some(TrapCodes::Halt),
            _ => None,
        }
    }
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

    pub fn write(&mut self, address: RegisterCodes, value: u16) {
        self.container[address as usize] = value
    }

    pub fn read(&self, address: RegisterCodes) -> u16 {
        self.container[address as usize]
    }
}

impl Registers {
    pub fn perform_branch(&mut self, instruction: u16) {
        let program_counter_offset = Registers::sign_extend(instruction & 0x1FF, 9);
        let condition_flag = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();

        if ((condition_flag as u16) & self.read(RegisterCodes::ConditionFlags)) == 0 {
            return;
        }

        let program_counter_value =
            self.read(RegisterCodes::ProgramCounter) + program_counter_offset;
        self.set_program_counter(program_counter_value);
    }

    pub fn perform_add(&mut self, instruction: u16) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let first_operand_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();
        let first_operand_register_value = self.read(first_operand_register);

        let new_value = {
            // whether we are in immediate mode
            let immediate_flag = (instruction >> 5) & 0x1;
            if immediate_flag != 0 {
                let immediate5 = Registers::sign_extend(instruction & 0x1F, 5);

                first_operand_register_value + immediate5
            } else {
                let second_operand_register = RegisterCodes::from(instruction & 0x7).unwrap();
                let second_operand_register_value = self.read(second_operand_register);

                first_operand_register_value + second_operand_register_value
            }
        };

        self.write(destination_register.clone(), new_value);
        self.update_flags(destination_register);
    }

    pub fn perform_load(&mut self, instruction: u16, memory: &mut Memory) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let program_counter_offset = Registers::sign_extend(instruction & 0x1FF, 9);
        let memory_address =
            (self.read(RegisterCodes::ProgramCounter) as u16) + program_counter_offset;
        let new_value = memory.read(memory_address);
        self.write(destination_register.clone(), new_value);
        self.update_flags(destination_register);
    }

    pub fn perform_store(&mut self, instruction: u16, memory: &mut Memory) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let program_counter_offset = Registers::sign_extend(instruction & 0x1FF, 9);
        let memory_address = self.read(RegisterCodes::ProgramCounter) + program_counter_offset;
        memory.write(memory_address, self.read(destination_register));
    }

    pub fn perform_jump(&mut self, instruction: u16) {
        let new_value_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();
        let new_value = self.read(new_value_register);
        self.set_program_counter(new_value);
    }

    pub fn perform_jump_register(&mut self, instruction: u16) {
        let program_counter = self.read(RegisterCodes::ProgramCounter);
        self.write(RegisterCodes::R7, program_counter as u16);

        let new_value = {
            let long_flag = (instruction >> 11) & 1;
            if long_flag != 0 {
                let long_program_counter_offset = Registers::sign_extend(instruction & 0x7FF, 11);
                let program_counter = self.read(RegisterCodes::ProgramCounter);

                program_counter + long_program_counter_offset
            } else {
                let first_operand_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();

                self.read(first_operand_register)
            }
        };

        self.set_program_counter(new_value);
    }

    pub fn perform_bitwise_and(&mut self, instruction: u16) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let first_operand_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();
        let first_operand_register_value = self.read(first_operand_register);

        let new_value = {
            // whether we are in immediate mode
            let immediate_flag = (instruction >> 5) & 0x1;
            if immediate_flag != 0 {
                let immediate5 = Registers::sign_extend(instruction & 0x1F, 5);

                first_operand_register_value & immediate5
            } else {
                let second_operand_register = RegisterCodes::from(instruction & 0x7).unwrap();
                let second_operand_register_value = self.read(second_operand_register);

                first_operand_register_value & second_operand_register_value
            }
        };

        self.write(destination_register.clone(), new_value);
        self.update_flags(destination_register);
    }

    pub fn perform_load_register(&mut self, instruction: u16, memory: &mut Memory) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let first_operand_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();
        let offset = Self::sign_extend(instruction & 0x3F, 6);
        let memory_address = self.read(first_operand_register) + offset;
        let new_value = memory.read(memory_address);
        self.write(destination_register.clone(), new_value);
        self.update_flags(destination_register);
    }

    pub fn perform_store_register(&mut self, instruction: u16, memory: &mut Memory) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let first_operand_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();
        let offset = Self::sign_extend(instruction & 0x3F, 6);
        let memory_address = self.read(first_operand_register) + offset;
        memory.write(memory_address, self.read(destination_register));
    }

    pub fn perform_unused(&self) {}

    pub fn perform_not(&mut self, instruction: u16) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let first_operand_register = RegisterCodes::from((instruction >> 6) & 0x7).unwrap();
        let new_value = !self.read(first_operand_register);
        self.write(destination_register, new_value)
    }

    pub fn perform_load_indirect(&mut self, instruction: u16, memory: &mut Memory) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let program_counter_offset = Registers::sign_extend(instruction & 0x1FF, 9);
        // add pc_offset to the current PC, look at that memory location to get the final address
        let memory_address =
            memory.read(self.read(RegisterCodes::ProgramCounter) + program_counter_offset);
        let memory_value = memory.read(memory_address);
        self.write(destination_register, memory_value);
    }

    pub fn perform_store_indirect(&mut self, instruction: u16, memory: &mut Memory) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let program_counter_offset = Self::sign_extend(instruction & 0x1FF, 9);
        let memory_to_read_address =
            self.read(RegisterCodes::ProgramCounter) + program_counter_offset;
        let memory_address = memory.read(memory_to_read_address);
        memory.write(memory_address, self.read(destination_register));
    }

    pub fn perform_reserved(&self) {}

    pub fn perform_load_effective_address(&mut self, instruction: u16) {
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        let program_counter_offset = Self::sign_extend(instruction & 0x1FF, 9);
        let new_value = self.read(RegisterCodes::ProgramCounter) + program_counter_offset;
        self.write(destination_register.clone(), new_value);
        self.update_flags(destination_register);
    }

    pub fn perform_execute_trap(&mut self, instruction: u16, memory: &mut Memory) {
        self.write(RegisterCodes::R7, self.read(RegisterCodes::ProgramCounter));

        let trap_code = match TrapCodes::from(instruction & 0xFF) {
            Some(value) => value,
            None => return,
        };
        match trap_code {
            TrapCodes::GetCharacter => self.perform_get_character_trap(),
            TrapCodes::OutputCharacter => self.perform_output_character_trap(),
            TrapCodes::OutputWord => self.perform_output_word_trap(memory),
            TrapCodes::EchoCharacterToTerminal => self.perform_echo_character_to_terminal_trap(),
            TrapCodes::OutputByteString => self.perform_output_byte_string_trap(),
            TrapCodes::Halt => self.perform_halt_trap(),
        }
    }
}

impl Registers {
    fn perform_get_character_trap(&self) {
        todo!()
    }

    fn perform_output_character_trap(&self) {
        todo!()
    }

    fn perform_output_word_trap(&mut self, memory: &mut Memory) {
        let mut memory_address = self.read(RegisterCodes::R0);
        let mut character = memory.read(memory_address);

        while character != 0x0000 {
            print!("{}", (character as u8) as char);

            memory_address += 1;
            character = memory.read(memory_address);
        }

        io::stdout().flush().expect("failed to flush");
    }

    fn perform_echo_character_to_terminal_trap(&self) {
        todo!()
    }

    fn perform_output_byte_string_trap(&self) {
        todo!()
    }

    fn perform_halt_trap(&self) {
        todo!()
    }
}

impl Registers {
    fn update_flags(&mut self, code: RegisterCodes) {
        let register = self.container[code as usize];
        if register == 0 {
            self.set_condition_flag(ConditionFlags::Zero);
            return;
        }

        if (register >> 15) != 0 {
            self.set_condition_flag(ConditionFlags::Negative);
            return;
        }

        self.set_condition_flag(ConditionFlags::Positive);
    }

    fn sign_extend(value: u16, bit_count: i32) -> u16 {
        let mut result = value;
        if ((value >> (bit_count - 1)) & 1) != 0 {
            result |= 0xFFFF << bit_count;
        }

        return result;
    }
}
