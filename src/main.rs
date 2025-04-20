#![allow(unused)]

pub mod alu;
pub mod decoder;
pub mod memory;
pub mod cpu;
pub mod opcodes;
pub mod register;
pub mod bitops;

use std::{env, process::ExitCode};
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

  let rom_file = "/home/cedar/Code/rust/mccm-emulator/roms/test_rom"; //env::args().nth(1).expect("Expected the rom filepath.");

  let mut rom = Memory::new(std::fs::read(&rom_file).expect("Failed to read rom file"));

  println!("{:#x?}", rom);

  loop {
    // println!("{:#?}", rom);
    // println!("{:#?}", regsiters);
    
    match cpu::run(&mut regsiters, &mut rom) {
      Some(code) => return code,
      None => continue,
    }
  }
}