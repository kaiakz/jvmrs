pub mod dup;
pub mod swap;
pub mod pop;

use crate::instructions::Instruction;

pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x57 => Box::new(pop::POP::init(1)),
        0x58 => Box::new(pop::POP::init(2)),
        0x59 => Box::new(dup::DUP::init(1, 0)),
        0x5a => Box::new(dup::DUP::init(1, 1)),
        0x5b => Box::new(dup::DUP::init(1, 2)),
        0x5c => Box::new(dup::DUP::init(2, 0)),
        0x5d => Box::new(dup::DUP::init(2, 1)),
        0x5e => Box::new(dup::DUP::init(2, 2)),
        0x5f => Box::new(swap::SWAP {}),
        _ => panic!("Instruction Error: stack"),
    }
}