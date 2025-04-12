use crate::{register::Register, Byte};

pub enum ImmediateFlag {
  No,
  Yes
}

fn no_immediate(in_val: usize, out_reg: usize, registers: &mut [Register]) {
  todo!("mov::no_immediate()")
}

fn yes_immediate(in_val: usize, out_reg: usize, registers: &mut [Register]) {
  todo!("mov::yes_immediate()")
}

pub fn apply(in_val: usize, out_reg:usize, flag: ImmediateFlag,  registers: &mut [Register]) {
  match flag {
    ImmediateFlag::No => no_immediate(in_val, out_reg, registers),
    ImmediateFlag::Yes => yes_immediate(in_val, out_reg, registers),
  }
}


