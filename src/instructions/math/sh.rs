use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;
use std::u32;
use std::u64;
use std::i32;
use std::i64;

// Despite the fact that overflow may occur, execution of an sh instruction never throws a run-time exception. 


pub struct ISH {
    pub left: bool, //true: ishl; false: ishr
}

impl Instruction for ISH {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: u32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val as u32,
            _ => panic!("value2 must be of type int."),
        };
        let lhs: i32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val,
            _ => panic!("value1 must be of type int."),
        };

        if self.left {
            frame.operand_stack_push(Slot::Int(lhs.wrapping_shl(rhs))); 
        } else {
            frame.operand_stack_push(Slot::Int(lhs.wrapping_shr(rhs)));
        }   
    }
}


pub struct LSH {
    pub left: bool, //true: lshl; false: lshr
}

impl Instruction for LSH {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: u32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val as u32,
            _ => panic!("value2 must be of type int."),
        };
        let lhs: i64 = match frame.operand_stack_pop() {
            Slot::Long(val) => val,
            _ => panic!("value1 must be of type long."),
        };

        if self.left {
            frame.operand_stack_push(Slot::Long(lhs.wrapping_shl(rhs)));
        } else {
            frame.operand_stack_push(Slot::Long(lhs.wrapping_shr(rhs)));            
        }
    }
}

pub struct IUSHR {}

impl Instruction for IUSHR {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: u32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val as u32,
            _ => panic!("value2 must be of type int."),
        };
        let lhs: u32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val as u32,
            _ => panic!("value1 must be of type int."),
        };

        frame.operand_stack_push(Slot::Int(lhs.wrapping_shr(rhs) as i32));
    }
}


pub struct LUSHR {}

impl Instruction for LUSHR {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs: u32 = match frame.operand_stack_pop() {
            Slot::Int(val) => val as u32,
            _ => panic!("value2 must be of type int."),
        };
        let lhs: u64 = match frame.operand_stack_pop() {
            Slot::Long(val) => val as u64,
            _ => panic!("value1 must be of type long."),
        };

        frame.operand_stack_push(Slot::Long(lhs.wrapping_shr(rhs) as i64));            
    }
}