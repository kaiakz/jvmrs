use std::cmp::Ordering;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct IFICMPEQ {
    offset: u32,
}

impl Instruction for IFICMPEQ {
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

pub struct IFICMPNE {
    offset: u32,
}

impl Instruction for IFICMPNE {
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

pub struct IFICMPLT {
    offset: u32,
}

impl Instruction for IFICMPLT {
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

        if lhs < rhs  {

        }
}

pub struct IFICMPLE {
    offset: u32,
}

impl Instruction for IFICMPLE {
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

        if lhs <= rhs  {

        }
}

pub struct IFICMPGT {
    offset: u32,
}

impl Instruction for IFICMPGT {
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

        if lhs > rhs  {

        }
}

pub struct IFICMPGE {
    offset: u32,
}

impl Instruction for IFICMPGE {
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

        if lhs >= rhs  {

        }
}
