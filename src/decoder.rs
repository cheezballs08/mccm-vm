use core::num;
use std::result;

use crate::opcodes::mov;

pub enum ALUDirective {
  Mov(usize, usize, mov::ImmediateFlag, usize)  
}

enum Opcode {
  Mov = 0b0000000100000001,
}

#[repr()]
pub struct DecodedOpcode {
  opcode: Opcode,
  args: Vec<u8>
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

fn get_bit(num: u64, bit_index: u8) -> u8 {
  ((num >> bit_index) & 1) as u8 
}

pub fn decode_opcode(bytes: [u8; 2]) -> DecodedOpcode {
  todo!()
}

pub fn decode_args(opcode: DecodedOpcode) -> ALUDirective {
  match opcode.opcode {
    Opcode::Mov => {
      todo!()
    } 
  }
}