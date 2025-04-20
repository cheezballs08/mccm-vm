use std::ops::Range;


pub fn eight_bytes_to_u64(bytes: [u8; 8]) -> u64 {
  let mut result: u64 = 0;

  for i in 0..8 {
    result |= (bytes[i] as u64) << (i * 8);
  }

  result
}

pub fn four_bytes_to_u32(bytes: [u8; 4]) -> u32 {
  let mut result: u32 = 0;

  for i in 0..4 {
    result |= (bytes[i] as u32) << (i * 8);
  }

  result  
}

pub fn two_bytes_to_u16(bytes: [u8; 2]) -> u16 {
  let mut result: u16 = 0;

  for i in 0..2 {
    result |= (bytes[i] as u16) << (i * 8);
  }

  result  
}

pub fn bytes_to_num(bytes: Vec<u8>) -> u64 {
  let width = bytes.len();

  match width {
    1 => {
      bytes.get(0).unwrap().clone() as u64
    }
    2 => {
      two_bytes_to_u16(bytes.try_into().unwrap()) as u64
    }
    4 => {
      four_bytes_to_u32(bytes.try_into().unwrap()) as u64
    }
    8 => {
      eight_bytes_to_u64(bytes.try_into().unwrap())
    }
    _ => {
      panic!("Invalid number of bytes: {}", width)
    }
  }
}

pub fn get_bit(num: usize, bit_index: u8) -> u8 {
  ((num >> bit_index) & 1) as u8 
}

pub fn get_bits(num: usize, range: Range<u8>) -> u8 {
  let mut result = 0;

  for i in range {
    result |= get_bit(num, i) << i;
  }

  result
}