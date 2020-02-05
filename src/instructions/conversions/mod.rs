pub mod i2x;
pub mod l2x;
pub mod f2x;
pub mod d2x;

use crate::instructions::Instruction;

pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x85 => Box::new(i2x::I2L {}),
        0x86 => Box::new(i2x::I2F {}),
        0x87 => Box::new(i2x::I2D {}),
        0x88 => Box::new(l2x::L2I {}),
        0x89 => Box::new(l2x::L2F {}),
        0x8a => Box::new(l2x::L2D {}),
        0x8b => Box::new(f2x::F2I {}),
        0x8c => Box::new(f2x::F2L {}),
        0x8d => Box::new(f2x::F2D {}),
        0x8e => Box::new(d2x::D2I {}),
        0x8f => Box::new(d2x::D2L {}), 
        0x90 => Box::new(d2x::D2F {}),
        0x91 => Box::new(i2x::I2B {}),
        0x92 => Box::new(i2x::I2C {}),
        0x93 => Box::new(i2x::I2S {}),
        _ => panic!("Instruction Error: conversions"),
    }

}