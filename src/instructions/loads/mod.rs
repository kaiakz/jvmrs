pub mod xload;

use crate::instructions::Instruction;

// opcode: 0x15~0x35
pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x15 => Box::new(xload::ILOAD::init(0, false)),
        0x16 => Box::new(xload::LLOAD::init(0, false)),
        0x17 => Box::new(xload::FLOAD::init(0, false)),
        0x18 => Box::new(xload::DLOAD::init(0, false)),
        0x19 => Box::new(xload::ALOAD::init(0, false)),
        0x1a => Box::new(xload::ILOAD::init(0, true)),
        0x1b => Box::new(xload::ILOAD::init(1, true)),
        0x1c => Box::new(xload::ILOAD::init(2, true)),
        0x1d => Box::new(xload::ILOAD::init(3, true)),
        0x1e => Box::new(xload::LLOAD::init(0, true)),
        0x1f => Box::new(xload::LLOAD::init(1, true)),
        0x20 => Box::new(xload::LLOAD::init(2, true)),
        0x21 => Box::new(xload::LLOAD::init(3, true)),
        0x22 => Box::new(xload::FLOAD::init(0, true)),
        0x23 => Box::new(xload::FLOAD::init(1, true)),
        0x24 => Box::new(xload::FLOAD::init(2, true)),
        0x25 => Box::new(xload::FLOAD::init(3, true)),
        0x26 => Box::new(xload::DLOAD::init(0, true)),
        0x27 => Box::new(xload::DLOAD::init(1, true)),
        0x28 => Box::new(xload::DLOAD::init(2, true)),
        0x29 => Box::new(xload::DLOAD::init(3, true)),
        0x2a => Box::new(xload::ALOAD::init(0, true)),
        0x2b => Box::new(xload::ALOAD::init(1, true)),
        0x2c => Box::new(xload::ALOAD::init(2, true)),
        0x2d => Box::new(xload::ALOAD::init(3, true)),
        // 0x2e => Box::new(xload::),
        // 0x2f => Box::new(xload::),
        _ => panic!("Instruction Error: loads"),
    }

}