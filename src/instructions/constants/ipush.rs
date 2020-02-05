use crate::mem::thread;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;


///opcode: 0x01~0x0f
pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x10 => Box::new(BIPUSH {value: 0}),
        0x11 => Box::new(SIPUSH {value: 0}),
        _ => panic!("Instruction Error: ipush"),
    }
    
}

pub struct BIPUSH {
    value: u8,
}

impl Instruction for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        self.value = reader.fetch_u8();
    }

    fn excute(&self, frame: &mut Frame) {
        frame.operand_stack_push(Type::Int(self.value as i32));
    }
}



pub struct SIPUSH {
    value: u16,
}

impl Instruction for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        self.value = reader.fetch_u16();
    }

    fn excute(&self, frame: &mut Frame) {
        frame.operand_stack_push(Type::Int(self.value as i32));
    }
}