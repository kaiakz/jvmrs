pub mod xstore;

use crate::instructions::Instruction;

pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x36 => Box::new(xstore::ISTORE::init(0, false)),
        0x37 => Box::new(xstore::LSTORE::init(0, false)),
        0x38 => Box::new(xstore::FSTORE::init(0, false)),
        0x39 => Box::new(xstore::DSTORE::init(0, false)),
        0x3a => Box::new(xstore::ASTORE::init(0, false)),
        0x3b => Box::new(xstore::ISTORE::init(0, true)),
        0x3c => Box::new(xstore::ISTORE::init(1, true)),
        0x3d => Box::new(xstore::ISTORE::init(2, true)),
        0x3e => Box::new(xstore::ISTORE::init(3, true)),
        0x3f => Box::new(xstore::LSTORE::init(0, true)),
        0x40 => Box::new(xstore::LSTORE::init(1, true)),
        0x41 => Box::new(xstore::LSTORE::init(2, true)),
        0x42 => Box::new(xstore::LSTORE::init(3, true)),
        0x43 => Box::new(xstore::FSTORE::init(0, true)),
        0x44 => Box::new(xstore::FSTORE::init(1, true)),
        0x45 => Box::new(xstore::FSTORE::init(2, true)),
        0x46 => Box::new(xstore::FSTORE::init(3, true)),
        0x47 => Box::new(xstore::DSTORE::init(0, true)),
        0x48 => Box::new(xstore::DSTORE::init(1, true)),
        0x49 => Box::new(xstore::DSTORE::init(2, true)),
        0x4a => Box::new(xstore::DSTORE::init(3, true)),
        0x4b => Box::new(xstore::ASTORE::init(0, true)),
        0x4c => Box::new(xstore::ASTORE::init(1, true)),
        0x4d => Box::new(xstore::ASTORE::init(2, true)),
        0x4e => Box::new(xstore::ASTORE::init(3, true)),
        _ => panic!("Instruction Error: stores"),
    }
}