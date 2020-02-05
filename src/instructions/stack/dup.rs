use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;
use super::pop;

// dup, dup_x1, dup_x2, dup2, dup2_x1, dup2_x2
pub struct DUP {
    dup_len: u8,    //1,2
    dup_x: u8,  //0,1,2
}

impl DUP {
    pub fn init(dup_len: u8, dup_x: u8) -> DUP {
        DUP {
            dup_len: dup_len,
            dup_x: dup_x,
        }
    }
}

impl Instruction for DUP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let mut first = if self.dup_len == 1 {
            pop::pop1(frame)
        } else {
            pop::pop2(frame)
        };
        let mut second = first.clone();
        
        if self.dup_x == 1 {
            second.extend(pop::pop1(frame));
        } else {
            second.extend(pop::pop2(frame));
        }
     
        frame.operand_stack_extend(&mut first);
        frame.operand_stack_extend(&mut second);
    }
}

// pub struct DUP2 {
//     dup_x: u8,
// }

// impl Instruction for DUP2 {
//     fn fetch_operands(&mut self, reader: &mut ByteCode) {}

//     fn excute(&self, frame: &mut Frame) {
//         let i = frame.operand_stack_pop();
//         frame.operand_stack_push(i);
//         frame.operand_stack_push(i);
//     }
// }