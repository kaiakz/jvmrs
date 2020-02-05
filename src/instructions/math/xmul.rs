use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;
use std::i32;
use std::i64;
use std::f32;
use std::f64;

// Despite the fact that overflow may occur, execution of an imul instruction never throws a run-time exception. 
pub struct IMUL {}

impl Instruction for IMUL {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: i32 = match frame.operand_stack_pop() {
            Type::Int(val) => val,
            _ => panic!("Both value1 and value2 must be of type int."),
        };
        let lhs: i32 = match frame.operand_stack_pop() {
            Type::Int(val) => val,
            _ => panic!("Both value1 and value2 must be of type int."),
        };

        frame.operand_stack_push(Type::Int(lhs * rhs));
        
    }
}

// Despite the fact that overflow may occur, execution of an lmul instruction never throws a run-time exception. 
pub struct LMUL {}

impl Instruction for LMUL {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: i64 = match frame.operand_stack_pop() {
            Type::Long(val) => val,
            _ => panic!("Both value1 and value2 must be of type long."),
        };
        let lhs: i64 = match frame.operand_stack_pop() {
            Type::Long(val) => val,
            _ => panic!("Both value1 and value2 must be of type long."),
        };

        frame.operand_stack_push(Type::Long(lhs * rhs));
        
    }
}


pub struct FMUL {}

impl Instruction for FMUL {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: f32 = match frame.operand_stack_pop() {
            Type::Float(val) => val,
            _ => panic!("Both value1 and value2 must be of type float."),
        };
        let lhs: f32 = match frame.operand_stack_pop() {
            Type::Float(val) => val,
            _ => panic!("Both value1 and value2 must be of type float."),
        };

        frame.operand_stack_push(Type::Float(lhs * rhs));
        
    }
}


pub struct DMUL {}

impl Instruction for DMUL {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: f64 = match frame.operand_stack_pop() {
            Type::Double(val) => val,
            _ => panic!("Both value1 and value2 must be of type double."),
        };
        let lhs: f64 = match frame.operand_stack_pop() {
            Type::Double(val) => val,
            _ => panic!("Both value1 and value2 must be of type double."),
        };

        frame.operand_stack_push(Type::Double(lhs * rhs));
        
    }
}