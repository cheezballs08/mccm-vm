use core::{num, panic};
use std::{fmt::format, ops::Range, result, u8, usize, vec};

use crate::opcodes::{add, mov::{self, ImmediateFlag}};
use crate::bitops;

#[derive(Debug)]
pub enum ALUDirective {
  Mov(usize, usize, mov::ImmediateFlag, usize),
  Add(usize, usize, usize, add::ImmediateFlag, usize),
}

impl ToString for ALUDirective {
  fn to_string(&self) -> String {
    match self {
      ALUDirective::Mov(in_val, out_reg, immediate_flag, offset) => {
        match immediate_flag {
          mov::ImmediateFlag::No => {
            format!("mov r{} -> r{}", in_val, out_reg)
          }
          mov::ImmediateFlag::Yes => {
            format!("mov::i{} #{} -> r{}", (offset - 3) * 8, in_val, out_reg)
          }
        }
      }
      ALUDirective::Add(in_val, out_reg, out_val, immediate_flag, offset) => {
        todo!("Decoding for add opcode not implemented yet.")
      }
    }
  }
}

#[repr(u16)]
enum Opcode {
  Mov(mov::ImmediateFlag) = 0b1,
  Add(add::ImmediateFlag) = 0b10,
}

pub struct DecodedOpcode {
  opcode: Opcode,
  required_args: u8
}

impl DecodedOpcode {
  pub fn required_args(&self) -> u8 {
    self.required_args
  }
}

pub fn decode_opcode(bytes: [u8; 2]) -> DecodedOpcode {
  let opcode_byte = bytes[1];

  let decorator_byte = bytes[0];

  match opcode_byte {
    1 => {
      let immediate_bit: u8 = bitops::get_bit(decorator_byte as usize, 7);
      let immediate_flag = match immediate_bit {
        0b0 => mov::ImmediateFlag::No,
        0b1 => mov::ImmediateFlag::Yes,
        _ => unreachable!()
      };

      let required_args: u8 = bitops::get_bits(decorator_byte as usize, 0..5);

      DecodedOpcode {
        opcode: Opcode::Mov(immediate_flag),
        required_args
      }
    }
    
    2 => {
      let immediate_bits = bitops::get_bits(decorator_byte as usize, 5..7);
      let immediate_flag = match immediate_bits {
        0b00 => add::ImmediateFlag::No,
        0b10 => add::ImmediateFlag::One,
        0b11 => add::ImmediateFlag::Two,
        _ => panic!("Invalid immediate flag state for add opcode. State: {:#04b}", immediate_bits) 
      };

      let required_args: u8 = bitops::get_bits(decorator_byte as usize, 0..5);

      DecodedOpcode {
        opcode: Opcode::Add(immediate_flag),
        required_args
      }
    }

    _ => {
      panic!("Opcode {:#010b} is not a vaild opcode.", opcode_byte)
    }
  }
}

pub fn decode_args(opcode: DecodedOpcode, requested_args: Vec<u8>) -> ALUDirective {
  match opcode.opcode {
    Opcode::Mov(immediate_flag) => {
      mov::decode_args(immediate_flag, requested_args)
    }

    Opcode::Add(immediate_flag) => {
      add::decode_args(immediate_flag, requested_args)
    }
  }
}