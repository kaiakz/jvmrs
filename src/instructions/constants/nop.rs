use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;

pub struct NOP {}

impl Instruction for NOP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {}
}