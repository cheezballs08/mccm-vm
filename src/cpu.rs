use std::process::{ExitCode};

use crate::{alu, decoder::{decode_args, decode_opcode}, memory::Memory, register::Register};

pub fn run(registers: &mut [Register], rom: &mut Memory) -> Option<ExitCode> {

  let opcode_byte: Option<&[u8]> = rom.get_bytes(2);
  
  let opcode = match opcode_byte {
    Some(byte) => decode_opcode([byte[0], byte[1]]),
    None => {
      println!("Reached end of rom, terminating execution, ptr: {}", rom.get_ptr());
      return Some(ExitCode::SUCCESS);
    },
  };

  rom.move_back_by_n(2);

  let args = rom.get_bytes(opcode.required_args() as usize).unwrap().to_vec();

  let directive = decode_args(opcode, args);

  alu::apply_operation(directive, registers, rom);
  
  None
}