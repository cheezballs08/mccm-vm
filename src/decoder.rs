use crate::opcodes::mov;

pub enum ALUDirective {
  Mov(usize, usize, mov::ImmediateFlag, usize)  
}

pub enum MCCMOperation {
  Mov = 0x0, 
}


fn four_bytes_to_u64(bytes: [u8; 4]) -> usize {
  let mut result: usize = 0;

  for i in 0..4 {
    result |= (bytes[i] as usize) << (i * 8);
  }

  result
}

fn get_bit(bit: u8) {
  todo!("decoder::get_bit()")
}

fn decode_opcode(opcode: usize) -> MCCMOperation {
  todo!("decoder::decode_opcode()")
}

fn decode_args(operation: MCCMOperation, bytes: [u8; 12]) -> ALUDirective {
  todo!("decoder::decode_args()")
}

pub fn decode_instruction(bytes: [u8; 16]) -> ALUDirective {
  let opcode: [u8; 4] = [
    bytes[0], 
    bytes[1], 
    bytes[2], 
    bytes[3]
  ];

  let potential_args = [
    bytes[4], 
    bytes[5], 
    bytes[6], 
    bytes[7], 
    bytes[8], 
    bytes[9], 
    bytes[10], 
    bytes[11], 
    bytes[12], 
    bytes[13], 
    bytes[14], 
    bytes[15]
  ];

  let opcode_u64 = four_bytes_to_u64(opcode);

  let opcode_type = decode_opcode(opcode_u64);

  match opcode_type {
    MCCMOperation::Mov => {
      decode_args(opcode_type, potential_args)
    }
  }
}

#[cfg(test)]
mod tests {
  
  use super::*;
  
  #[test]
  fn test_four_bytes_to_u64() {
    let bytes = [0b00000001, 0b00000001, 0b00000001, 0b00000001];
  
    let result = four_bytes_to_u64(bytes);

    assert_eq!(result, 0b00000001000000010000000100000001);
  }
}