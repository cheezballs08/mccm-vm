use crate::{register::Register, Byte};

#[derive(Debug)]
pub enum ImmediateFlag {
  No,
  Yes
}

fn no_immediate(in_val: usize, out_reg: usize, registers: &mut [Register]) {
  registers[out_reg].set(in_val);
}

fn yes_immediate(in_val: usize, out_reg: usize, registers: &mut [Register]) {

}

pub fn apply(in_val: usize, out_reg:usize, flag: ImmediateFlag,  registers: &mut [Register]) {
  match flag {
    ImmediateFlag::No => no_immediate(in_val, out_reg, registers),
    ImmediateFlag::Yes => yes_immediate(in_val, out_reg, registers),
  }
}


