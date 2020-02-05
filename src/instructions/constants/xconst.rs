use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct XCONST {
    value: Slot,
}

///opcode: 0x01~0x0f
pub fn new(opcode: u8) -> Box<dyn Instruction> {
    let val: Slot = match opcode {
        0x01 => Slot::Ref(None),    //aconst_null
        0x02 => Slot::Int(-1),      //iconst_m1
        0x03 => Slot::Int(0),
        0x04 => Slot::Int(1),
        0x05 => Slot::Int(2),
        0x06 => Slot::Int(3),
        0x07 => Slot::Int(4),
        0x08 => Slot::Int(5),
        0x09 => Slot::Long(0),
        0x0a => Slot::Long(1),
        0x0b => Slot::Float(0.0),
        0x0c => Slot::Float(1.0),
        0x0d => Slot::Float(2.0),
        0x0e => Slot::Double(0.0),
        0x0f => Slot::Double(1.0),
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