#[derive(Debug)]
pub struct Register {
  byte_repr: u8,
  val: u8
}

impl Register {
  pub const fn new(repr: u8, init_val: u8) -> Self {
    return Register {
      byte_repr: repr,
      val: init_val,
    }
  }

  pub fn word(&self) -> u8 {
    return self.byte_repr
  }

  pub fn set(&mut self, val: u8) {
    self.val = val
  }

  pub fn get(&self) -> u8 {
    self.val
  }
}

impl PartialEq for Register {
  fn eq(&self, other: &Self) -> bool {
    if self.byte_repr == other.byte_repr {
      return true;
    } 

    false
  }
}

impl Default for Register {
  fn default() -> Self {
    return Register {
      byte_repr: 0,
      val: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::register::Register;

  #[test]
  fn check_default() {
    let default_reg = Register::default();

    assert_eq!(default_reg, Register { byte_repr: 0, val: 0 })
  }

  #[test]
  fn check_get() {
    let reg = Register::new(0, 0);

    assert_eq!(reg.get(), 0)
  }

  #[test]
  fn check_set() {
    let mut reg = Register::default();

    reg.set(0);

    assert_eq!(reg.get(), 0);
  }
}
