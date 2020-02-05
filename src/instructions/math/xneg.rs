use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;
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
            Type::Int(val) => val,
            _ => panic!("value must be of type int."),
        };

        frame.operand_stack_push(Type::Int(-lhs));
        
    }
}


pub struct LNEG {}

impl Instruction for LNEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: i64 = match frame.operand_stack_pop() {
            Type::Long(val) => val,
            _ => panic!("value must be of type long."),
        };

        frame.operand_stack_push(Type::Long(-lhs));
        
    }
}


pub struct FNEG {}

impl Instruction for FNEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: f32 = match frame.operand_stack_pop() {
            Type::Float(val) => val,
            _ => panic!("value must be of type float."),
        };

        frame.operand_stack_push(Type::Float(-lhs));
        
    }
}


pub struct DNEG {}

impl Instruction for DNEG {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {

        let lhs: f64 = match frame.operand_stack_pop() {
            Type::Double(val) => val,
            _ => panic!("value must be of type double."),
        };

        frame.operand_stack_push(Type::Double(-lhs));
        
    }
}