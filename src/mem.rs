use crate::Byte;

#[derive(Debug)]
pub struct Memory {
  vec: Vec<Byte>,
  ptr: usize,
}

impl Memory {
  pub fn new(vec: Vec<Byte>) -> Self {
    Memory { vec, ptr: 0 }
  } 

  pub fn size(&self) -> usize {
    self.vec.len()
  }

  pub fn inc_ptr(&mut self) {
    self.ptr += 1
  }

  pub fn dec_ptr(&mut self) {
    self.ptr -= 1
  }

  pub fn set_ptr(&mut self, new_ptr: usize) {
    self.ptr = new_ptr
  }

  pub fn get_ptr(&self) -> usize {
    self.ptr
  }

  pub fn move_front_by_n(&mut self, n: usize) {
    for _i in 0..n {
      self.inc_ptr();
    }
  }

  pub fn move_back_by_n(&mut self, n: usize) {
    for _i in 0..n {
      self.dec_ptr();
    }
  }

  pub fn inc(&mut self) {
    *self.vec.get_mut(self.ptr).unwrap() += 1 
  }

  pub fn dec(&mut self) {
    *self.vec.get_mut(self.ptr).unwrap() -= 1 
  }

  pub fn set(&mut self, val: Byte) {
    *self.vec.get_mut(self.ptr).unwrap() = val 
  }

  pub fn get_bytes(&self, size: usize) -> Option<&[u8]> {
    self.vec.windows(size).nth(self.ptr)
  }

  pub fn set_vec(&mut self, vec: Vec<Byte>) {
    self.vec = vec
  }
}

impl Default for Memory {
  fn default() -> Self {
    Memory::new(Vec::new())
  }
}