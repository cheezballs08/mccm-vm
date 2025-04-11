use motherboard::{opcode::Operation, register::Register};

use crate::{mem::Memory, Byte};


pub struct CPU {
  registers: Vec<Register>,
  opcodes: Vec<Box<dyn Operation>>,
  rom: Memory
}

impl CPU {
  pub fn run(&mut self) -> bool {
    todo!()
  }

  pub fn can_run(&self) -> bool {
    self.rom.get_ptr() < self.rom.size() 
  }

  pub fn add_opcodes(&mut self, mut opcodes: Vec<Box<dyn Operation>>) {
    self.opcodes.append(&mut opcodes);
  }

  pub fn set_rom(&mut self, rom: Vec<Byte>) {
    self.rom.set_vec(rom);
  }

  pub fn get_registers(&self) -> &Vec<Register> {
    &self.registers
  }

  pub fn get_rom(&self) -> &Memory {
    &self.rom
  }
}

impl Default for CPU {
  fn default() -> Self {
      CPU { 
        registers: vec![
          Register::new(0x0, 0),
          Register::new(0x1, 0),
          Register::new(0x2, 0),
          Register::new(0x3, 0),
          Register::new(0x4, 0),
          Register::new(0x5, 0),
          Register::new(0x6, 0),
          Register::new(0x7, 0),
        ], 
        opcodes: Vec::new(),
        rom: Memory::default()
      }
  }
}