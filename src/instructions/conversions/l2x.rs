use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct L2I {}

impl Instruction for L2I {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Long(val) => frame.operand_stack_push(Slot::Int(val as i32)),
            _ => panic!("l2x only support type Long"),
        };
    }
}

pub struct L2F {}

impl Instruction for L2F {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Long(val) => frame.operand_stack_push(Slot::Float(val as f32)),
            _ => panic!("l2x only support type Long"),
        };
    }
}

pub struct L2D {}

impl Instruction for L2D {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Long(val) => frame.operand_stack_push(Slot::Double(val as f64)),
            _ => panic!("l2x only support type Long"),
        };
    }
}
