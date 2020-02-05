use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;
use super::pop;

pub struct SWAP {}

impl Instruction for SWAP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        if let Some(first) = pop::pop1(frame).pop() {
            if let Some(second) = pop::pop1(frame).pop() {
                frame.operand_stack_push(first);
                frame.operand_stack_push(second);                
            }
        }
    }
}