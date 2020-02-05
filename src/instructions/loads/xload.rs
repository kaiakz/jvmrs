use crate::mem::thread;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;

pub struct XLOAD {
    index: usize,
    const_flag: bool,
}

impl XLOAD {
    pub fn init(index: usize, const_flag: bool) -> XLOAD {
        XLOAD {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for XLOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.local_variables_get(self.index);
        frame.operand_stack_push(i);
    }
}