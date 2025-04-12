use core::num;
use std::{ops::Range, result, u8, vec};

use crate::opcodes::mov::{self, ImmediateFlag};

pub enum ALUDirective {
  Mov(usize, usize, mov::ImmediateFlag, usize)  
}

#[repr(u16)]
enum Opcode {
  Mov(mov::ImmediateFlag) = 0b0000000100000001,
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

fn eight_bytes_to_u64(bytes: [u8; 8]) -> u64 {
  let mut result: u64 = 0;

  for i in 0..8 {
    result |= (bytes[i] as u64) << (i * 8);
  }

  result
}

fn four_bytes_to_u32(bytes: [u8; 4]) -> u32 {
  let mut result: u32 = 0;

  for i in 0..4 {
    result |= (bytes[i] as u32) << (i * 8);
  }

  result  
}

fn two_bytes_to_u16(bytes: [u8; 2]) -> u16 {
  let mut result: u16 = 0;

  for i in 0..2 {
    result |= (bytes[i] as u16) << (i * 8);
  }

  result  
}

fn bytes_to_num(bytes: Vec<u8>) -> u64 {
  let width = bytes.len();

  match width {
    1 => {
      bytes.get(0).unwrap().clone() as u64
    }
    2 => {
      two_bytes_to_u16(bytes.try_into().unwrap()) as u64
    }
    4 => {
      four_bytes_to_u32(bytes.try_into().unwrap()) as u64
    }
    8 => {
      eight_bytes_to_u64(bytes.try_into().unwrap())
    }
    _ => {
      unreachable!("Invalid number of bytes")
    }
  }
}

fn get_bit(num: usize, bit_index: u8) -> u8 {
  ((num >> bit_index) & 1) as u8 
}

fn get_bits(num: usize, range: Range<u8>) -> u8 {
  todo!()
}

pub fn decode_opcode(bytes: [u8; 2]) -> DecodedOpcode {
  let opcode_byte = bytes[0];

  let decorator_byte = bytes[1];

  match opcode_byte {
    val if val == Opcode::Mov as u8 => {
      let immediate_bit: u8 = get_bit(decorator_byte as usize, 7);
      let immediate_flag = match immediate_bit {
        0 => mov::ImmediateFlag::No,
        1 => mov::ImmediateFlag::Yes,
        _ => unreachable!()
      };

      let required_args: u8 = get_bits(decorator_byte as usize, 6..4);

      DecodedOpcode {
        opcode: Opcode::Mov(immediate_flag),
        required_args
      }
    }
    _ => {
      todo!()
    }
  }
}

pub fn decode_args(opcode: DecodedOpcode, requested_args: Vec<u8>) -> ALUDirective {
  match opcode.opcode {
    Opcode::Mov(immediate_flag) => {
      match immediate_flag {
        mov::ImmediateFlag::No => {
          let in_reg = requested_args[0];

          let out_reg = requested_args[1];

          return ALUDirective::Mov(in_reg as usize, out_reg as usize, immediate_flag, 4);

        }
        mov::ImmediateFlag::Yes => {
          let in_reg = requested_args[0];

          let arg_amount = requested_args.len();

          let out_reg = requested_args.get(arg_amount-1..1).unwrap();

          let immediate_value = bytes_to_num(requested_args.get(1..arg_amount-1).unwrap().to_vec());

          return ALUDirective::Mov(in_reg as usize, out_reg[0] as usize, immediate_flag, immediate_value as usize);
        }
      }
    } 
    _ => {
      todo!()
    }
  }
}