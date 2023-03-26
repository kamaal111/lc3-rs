#[path = "./condition_flags.rs"]
pub mod condition_flags;
use condition_flags::ConditionFlags;

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
    pub fn add(&mut self, instruction: u16) {
        // destination register (DR)
        let destination_register = RegisterCodes::from((instruction >> 9) & 0x7).unwrap();
        // first operand (SR1)
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
