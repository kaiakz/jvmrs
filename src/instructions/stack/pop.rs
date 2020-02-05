use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub fn pop2(frame: &mut Frame) -> Vec<Slot> {
    let mut v: Vec<Slot> = Vec::new();
    v.push(frame.operand_stack_pop());
    match v[0] {
            Slot::Long(_val) => {},
            Slot::Double(_val) => {},
            _ => {v.push(frame.operand_stack_pop());},
    };
    v
}

pub fn pop1(frame: &mut Frame) -> Vec<Slot> {
    let mut v: Vec<Slot> = Vec::new();
    v.push(frame.operand_stack_pop());
    match v[0] {
        Slot::Long(_val) => {panic!("pop does not support LONG")},
        Slot::Double(_val) => {panic!("pop does not support DOUBLE")},
        _ => v,
    }
}

pub struct POP {
    pop_len: u8, //1,2
}

impl POP {
    pub fn init(pop_len: u8) -> POP {
        POP {
            pop_len: pop_len,
        }
    }
}

impl Instruction for POP {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        if self.pop_len == 1 {
            pop1(frame);
        } else {
            pop2(frame);
        }
    }
}

// pub struct POP2 {}

// impl Instruction for POP2 {
//     fn fetch_operands(&mut self, reader: &mut ByteCode) {}

//     fn excute(&self, frame: &mut Frame) {
//         match frame.operand_stack_pop() {
//             Type::Long(_val) => {},
//             Type::Double(_val) => {},
//             _ => {frame.operand_stack_pop();},
//         };
//     }
// }