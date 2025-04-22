use crate::{decoder::ALUDirective, memory::Memory, opcodes::*, register::Register};

pub fn apply_operation(directive: ALUDirective, registers: &mut [Register], rom: &mut Memory) {
  println!("executing: {}", directive.to_string());

  match directive {
    ALUDirective::Mov(in_val, out_reg, immediate_flag, offset) => {
      mov::apply(in_val, out_reg, immediate_flag, registers);
      rom.move_front_by_n(offset);
    },
  }
}