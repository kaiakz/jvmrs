use std::cmp::Ordering;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;


pub struct LCMP {}

impl Instruction for LCMP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs = match frame.operand_stack_pop() {
            Slot::Long(value) => value,
            _ => panic!("lcmp only supports type Long"),
        };
        let lhs = match frame.operand_stack_pop() {
            Slot::Long(value) => value,
            _ => panic!("lcmp only supports type Long"),
        };

        match lhs.cmp(&rhs) {
            Ordering::Greater => frame.operand_stack_push(Slot::Int(1)),
            Ordering::Equal => frame.operand_stack_push(Slot::Int(0)),
            Ordering::Less => frame.operand_stack_push(Slot::Int(-1)),
        }
    }
}

pub struct FCMP {
    pub(super) default: i32,    // The fcmpg instruction pushes the int value 1 onto the operand stack and the fcmpl instruction pushes the int value -1 onto the operand stack.
}

impl Instruction for FCMP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs = match frame.operand_stack_pop() {
            Slot::Float(value) => value,
            _ => panic!("fcmpx only supports type Float"),
        };
        let lhs = match frame.operand_stack_pop() {
            Slot::Float(value) => value,
            _ => panic!("fcmpx only supports type Float"),
        };

        if lhs.is_nan() || rhs.is_nan() {
            frame.operand_stack_push(Slot::Int(self.default));
        }

        match lhs.cmp(&rhs) {
            Ordering::Greater => frame.operand_stack_push(Slot::Int(1)),
            Ordering::Equal => frame.operand_stack_push(Slot::Int(0)),
            Ordering::Less => frame.operand_stack_push(Slot::Int(-1)),
        }
    }
}

pub struct DCMP {
    pub(super) default: i32,    // The dcmpg instruction pushes the int value 1 onto the operand stack and the dcmpl instruction pushes the int value -1 onto the operand stack.
}

impl Instruction for DCMP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let rhs = match frame.operand_stack_pop() {
            Slot::Double(value) => value,
            _ => panic!("fcmpx only supports type Double"),
        };
        let lhs = match frame.operand_stack_pop() {
            Slot::Double(value) => value,
            _ => panic!("fcmpx only supports type Double"),
        };

        if lhs.is_nan() || rhs.is_nan() {
            frame.operand_stack_push(Slot::Int(self.default));
        }

        match lhs.cmp(&rhs) {
            Ordering::Greater => frame.operand_stack_push(Slot::Int(1)),
            Ordering::Equal => frame.operand_stack_push(Slot::Int(0)),
            Ordering::Less => frame.operand_stack_push(Slot::Int(-1)),
        }
    }
}