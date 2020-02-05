use crate::mem::thread;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::thread::Frame;
use crate::mem::thread::Type;

pub struct XCONST {
    value: Type,
}

///opcode: 0x01~0x0f
pub fn new(opcode: u8) -> Box<dyn Instruction> {
    let val: Type = match opcode {
        0x01 => Type::Ref(None),    //aconst_null
        0x02 => Type::Int(-1),      //iconst_m1
        0x03 => Type::Int(0),
        0x04 => Type::Int(1),
        0x05 => Type::Int(2),
        0x06 => Type::Int(3),
        0x07 => Type::Int(4),
        0x08 => Type::Int(5),
        0x09 => Type::Long(0),
        0x0a => Type::Long(1),
        0x0b => Type::Float(0.0),
        0x0c => Type::Float(1.0),
        0x0d => Type::Float(2.0),
        0x0e => Type::Double(0.0),
        0x0f => Type::Double(1.0),
        _ => panic!("Instruction Error: xconst"),
    };
    Box::new(XCONST {value: val})
}

impl Instruction for XCONST {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        frame.operand_stack_push(self.value.clone());
    }
}