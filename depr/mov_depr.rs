use motherboard::{opcode::Operation, register::Register};

use crate::Byte;

pub enum ImmediateFlag {
  No,
  Yes
}

pub struct Mov {
  byte_repr: Byte,
  flag: ImmediateFlag,
  word_amount: Byte
}

impl Mov {
  pub fn new(byte_repr: Byte, flag: ImmediateFlag) -> Self {

    match flag {
      ImmediateFlag::No => {
        Mov {
          byte_repr,
          flag,
          word_amount: 3,
        }
      },
      ImmediateFlag::Yes => {
        Mov {
          byte_repr,
          flag,
          word_amount: 3,
        }
      }
    }
  }

  fn no_immediate(&self, words: &[Byte], registers: &mut [Register]) {
    let reg = registers.iter().filter(|reg| reg.word() == words[1]).nth(0).unwrap();

    let in_val = reg.get();

    let reg = registers.iter_mut().filter(|reg| reg.word() == words[2]).nth(0).unwrap();

    reg.set(in_val);
  }

  fn yes_immediate(&self, words: &[Byte], registers: &mut [Register]) {

    let in_val = words[1];

    let reg = registers.iter_mut().filter(|reg| reg.word() == words[2]).nth(0).unwrap();

    reg.set(in_val);
  
  }

}

impl Operation for Mov {

  fn apply(&self, words: &[Byte], registers: &mut [Register]){

    match self.flag {
      ImmediateFlag::No => self.no_immediate(words, registers),
      ImmediateFlag::Yes => self.yes_immediate(words, registers),
    }
  }

  fn get_word_amount(&self) -> Byte {
    self.word_amount
  }

  fn word(&self) -> Byte {
    self.byte_repr
  }
}