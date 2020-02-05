use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;
use std::i32;
use std::i64;

pub enum BoolType {
    And,
    Or,
    Xor,
}

pub struct IBOOL {
    pub bt: BoolType,
}

impl Instruction for IBOOL {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: i32 = match frame.operand_stack_pop() {
            Type::Int(val) => val,
            _ => panic!("value2 must be of type int."),
        };
        let lhs: i32 = match frame.operand_stack_pop() {
            Type::Int(val) => val,
            _ => panic!("value1 must be of type int."),
        };

        let result: i32 = match self.bt {
            BoolType::And => lhs & rhs,
            BoolType::Or => lhs | rhs,
            BoolType::Xor => lhs ^ rhs
        };
        
        frame.operand_stack_push(Type::Int(result)); 
    }
}


pub struct LBOOL {
    pub bt: BoolType,
}

impl Instruction for LBOOL {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: i64 = match frame.operand_stack_pop() {
            Type::Long(val) => val,
            _ => panic!("value2 must be of type long."),
        };
        let lhs: i64 = match frame.operand_stack_pop() {
            Type::Long(val) => val,
            _ => panic!("value1 must be of type long."),
        };

        let result: i64 = match self.bt {
            BoolType::And => lhs & rhs,
            BoolType::Or => lhs | rhs,
            BoolType::Xor => lhs ^ rhs
        };

        frame.operand_stack_push(Type::Long(result));            

    }
}