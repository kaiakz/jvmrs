pub mod xload;

use crate::instructions::Instruction;

// opcode: 0x15~0x35
pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x15..=0x019 => Box::new(xload::XLOAD::init(0, false)),
        0x01..=0x0f => Box::new(xload::XLOAD::init(0, false)),
        0x10 | 0x11 => ipush::new(opcode),
        // 0x12 => ,
        // 0x13 => ,
        // 0x14 => ,
        _ => panic!("Instruction Error: loads"),
    }

}