use crate::{decoder::{ALUDirective, DecodedOpcode}, register::Register, Byte};

#[derive(Debug)]
pub enum ImmediateFlag {
  No,
  One,
  Two
}

pub fn no_immediate(in_reg: usize, in_reg2: usize, out_reg: usize, registers: &mut [Register]) {
  registers[out_reg].set(registers[in_reg].get().wrapping_add_signed(registers[in_reg2].get() as isize));
}

pub fn one_immediate(in_reg: usize, in_val: usize, out_reg: usize, registers: &mut [Register]) {
  registers[out_reg].set(registers[in_reg].get().wrapping_add_signed(in_val as isize));
}

pub fn two_immediate(in_val: usize, in_val2: usize, out_reg: usize, registers: &mut [Register]) {
  registers[out_reg].set(in_val.wrapping_add_signed(in_val2 as isize));
}

pub fn apply(in_val: usize, in_val2: usize, out_reg: usize, flag: ImmediateFlag,  registers: &mut [Register]) {
  match flag {
    ImmediateFlag::No => no_immediate(in_val, in_val2, out_reg, registers),
    ImmediateFlag::One => one_immediate(in_val, in_val2, out_reg, registers),
    ImmediateFlag::Two => two_immediate(in_val, in_val2, out_reg, registers),
  }
}

pub fn decode_args(opcode: ImmediateFlag, requested_args: Vec<u8>) -> ALUDirective {
  todo!("add::decode_args()")
}


