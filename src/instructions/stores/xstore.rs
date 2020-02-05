use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x36..=0x39 => Box::new(XLOAD {index: 0, const_flag: false}),
        0x3b | 0x3f | 0x43 | 0x47 | 0x4b => Box::new(XLOAD {index: 0, const_flag: true}),
        0x3c | 0x40 | 0x44 | 0x48 | 0x4c => Box::new(XLOAD {index: 1, const_flag: true}),
        0x3d | 0x41 | 0x45 | 0x49 | 0x4d => Box::new(XLOAD {index: 2, const_flag: true}),
        0x3e | 0x42 | 0x46 | 0x4a | 0x4e => Box::new(XLOAD {index: 3, const_flag: true}),
        _ => panic!("Instruction Error: xload"),
    }
}

pub struct XLOAD {
    index: usize,
    const_flag: bool,
}

impl Instruction for XLOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        frame.local_variables_set(self.index, i);
    }
}