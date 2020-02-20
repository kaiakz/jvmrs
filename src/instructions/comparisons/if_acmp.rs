use std::cmp::Ordering;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct IFACMPEQ {
    offset: u32,
}

impl Instruction for IFACMPEQ {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("lcmp only supports type Int"),
        };
        let lhs = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("lcmp only supports type Int"),
        };

        if lhs == rhs  {

        }
}

pub struct IFACMPNE {
    offset: u32,
}

impl Instruction for IFACMPNE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("lcmp only supports type Int"),
        };
        let lhs = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("lcmp only supports type Int"),
        };

        if lhs != rhs  {

        }
}