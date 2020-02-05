use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;
use std::i32;


pub struct IINC {
    index: usize,
    consta: i32,
}

impl IINC {
    pub fn new() -> IINC {
        IINC {
            index: 0,
            consta: 0,
        }
    }
}

impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        self.index = reader.fetch_u8() as usize;
        self.consta = reader.fetch_u8() as i32;
    }

    fn excute(&self, frame: &mut Frame) {
        let result = match frame.local_variables_get(self.index) {
            Slot::Int(val) => val + self.consta,
            _ => panic!("The local variable at index must contain an int."),
        };
        frame.local_variables_set(self.index, Slot::Int(result));
    }
}