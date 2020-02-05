pub mod xconst;
pub mod ipush;
pub mod nop;

use crate::instructions::Instruction;


///opcode: 0x01~0x14
pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x00 => Box::new(nop::NOP {}),
        0x01..=0x0f => xconst::new(opcode),
        0x10 | 0x11 => ipush::new(opcode),
        // 0x12 => ,
        // 0x13 => ,
        // 0x14 => ,
        _ => panic!("Instruction Error: constant"),
    }

}