use motherboard::register::Register;

use crate::{decoder::ALUDirective, mem::Memory, opcodes::*};

fn apply_operation(directive: ALUDirective, registers: &mut [Register], rom: &mut Memory) {
  match directive {
    ALUDirective::Mov(in_val, out_reg, immediate_flag, offset) => {
      mov::apply(in_val, out_reg, immediate_flag, registers);
      rom.move_front_by_n(offset);
    },
  }
}