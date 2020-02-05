use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct F2I {}

impl Instruction for F2I {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Float(val) => frame.operand_stack_push(Slot::Int(val as i32)),
            _ => panic!("f2x only support type Float"),
        };
    }
}

pub struct F2L {}

impl Instruction for F2L {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Float(val) => frame.operand_stack_push(Slot::Long(val as i64)),
            _ => panic!("f2x only support type Float"),
        };
    }
}

pub struct F2D {}

impl Instruction for F2D {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Float(val) => frame.operand_stack_push(Slot::Double(val as f64)),
            _ => panic!("f2x only support type Float"),
        };
    }
}