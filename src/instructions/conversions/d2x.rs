use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct D2I {}

impl Instruction for D2I {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Double(val) => frame.operand_stack_push(Slot::Int(val as i32)),
            _ => panic!("d2x only support type Double"),
        };
    }
}

pub struct D2L {}

impl Instruction for D2L {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Double(val) => frame.operand_stack_push(Slot::Long(val as i64)),
            _ => panic!("d2x only support type Double"),
        };
    }
}

pub struct D2F {}

impl Instruction for D2F {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Double(val) => frame.operand_stack_push(Slot::Float(val as f32)),
            _ => panic!("d2x only support type Double"),
        };
    }
}