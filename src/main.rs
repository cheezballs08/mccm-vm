#![allow(unused)]

pub mod alu;
pub mod decoder;
pub mod memory;
pub mod cpu;
pub mod opcodes;
pub mod register;

use std::process::ExitCode;

use memory::Memory;
use register::Register;

type Byte = u8;

fn main() -> ExitCode {

  let mut regsiters = [
    Register::new(0, 0),
    Register::new(1, 0),
    Register::new(2, 0),
    Register::new(3, 0),
    Register::new(4, 0),
    Register::new(5, 0),
    Register::new(6, 0),
    Register::new(7, 0),
    Register::new(8, 0),
    Register::new(9, 0),
    Register::new(10, 0),
    Register::new(11, 0),
    Register::new(12, 0),
    Register::new(13, 0),
    Register::new(14, 0),
    Register::new(15, 0),
  ];

  let mut rom = Memory::new(vec![
    0b00000010,
    0b1,
    0x5,
    0x7,
    0b00000010,
    0b1,
    0x10,
    0x3,
  ]);

  loop {
    println!("{:#?}", rom);
    println!("{:#?}", regsiters);
    
    match cpu::run(&mut regsiters, &mut rom) {
      Some(code) => return code,
      None => continue,
    }
  }
}
