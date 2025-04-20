use crate::{bitops, decoder::{ALUDirective, DecodedOpcode}, register::Register, Byte};

#[derive(Debug)]
pub enum ImmediateFlag {
  No,
  Yes
}

fn no_immediate(in_val: usize, out_reg: usize, registers: &mut [Register]) {
  registers[out_reg].set(registers[in_val].get());
}

fn yes_immediate(in_val: usize, out_reg: usize, registers: &mut [Register]) {
  registers[out_reg].set(in_val);
}

pub fn apply(in_val: usize, out_reg: usize, flag: ImmediateFlag,  registers: &mut [Register]) {
  match flag {
    ImmediateFlag::No => no_immediate(in_val, out_reg, registers),
    ImmediateFlag::Yes => yes_immediate(in_val, out_reg, registers),
  }
}

pub fn decode_args(immediate_flag: ImmediateFlag, requested_args: Vec<u8>) -> ALUDirective {
  match immediate_flag {
    ImmediateFlag::No => {
      let in_reg = requested_args[0] as usize;

      let out_reg = requested_args[1] as usize;

      return ALUDirective::Mov(in_reg, out_reg, immediate_flag, 4);
    }
    ImmediateFlag::Yes => {
      let arg_amount = requested_args.len();

      let out_reg = requested_args.last().unwrap();

      let mut args = requested_args.get(0..arg_amount - 1).unwrap().to_vec();
      args.reverse();

      let immediate_value = bitops::bytes_to_num(args);

      return ALUDirective::Mov(immediate_value as usize, *out_reg as usize, immediate_flag, arg_amount + 2);
    }
  }  
}