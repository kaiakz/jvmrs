use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;
use std::i32;
use std::i64;
use std::f32;
use std::f64;

pub struct IREM {}

impl Instruction for IREM {
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

        frame.operand_stack_push(Type::Int(lhs.wrapping_rem(rhs)));
        
    }
}


pub struct LREM {}

impl Instruction for LREM {
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

        frame.operand_stack_push(Type::Long(lhs.wrapping_rem(rhs)));
        
    }
}


pub struct FREM {}

impl Instruction for FREM {
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

        frame.operand_stack_push(Type::Float(lhs.rem_euclid(rhs)));
        
    }
}


pub struct DREM {}

impl Instruction for DREM {
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

        frame.operand_stack_push(Type::Double(lhs.rem_euclid(rhs)));
        
    }
}