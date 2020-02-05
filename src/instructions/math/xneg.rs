use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;
use std::i32;
use std::i64;
use std::f32;
use std::f64;

/// Unstable

pub struct INEG {}

impl Instruction for INEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: i32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val,
            _ => panic!("value must be of type int."),
        };

        frame.operand_stack_push(Slot::Int(-lhs));
        
    }
}


pub struct LNEG {}

impl Instruction for LNEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: i64 = match frame.operand_stack_pop() {
            Slot::Long(val) => val,
            _ => panic!("value must be of type long."),
        };

        frame.operand_stack_push(Slot::Long(-lhs));
        
    }
}


pub struct FNEG {}

impl Instruction for FNEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: f32 = match frame.operand_stack_pop() {
            Slot::Float(val) => val,
            _ => panic!("value must be of type float."),
        };

        frame.operand_stack_push(Slot::Float(-lhs));
        
    }
}


pub struct DNEG {}

impl Instruction for DNEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: f64 = match frame.operand_stack_pop() {
            Slot::Double(val) => val,
            _ => panic!("value must be of type double."),
        };

        frame.operand_stack_push(Slot::Double(-lhs));
        
    }
}