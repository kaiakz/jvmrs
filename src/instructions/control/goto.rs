use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct GOTO {
    offset usize,
}

impl Instruction for GOTO {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        self.offset = reader.fetch_u16() as usize;
    }

    fn excute(&self, frame: &mut Frame) {

    }
}