use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct I2L {}

impl Instruction for I2L {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(val) => frame.operand_stack_push(Slot::Long(val as i64)),
            _ => panic!("i2x only support type Int"),
        };
    }
}

pub struct I2F {}

impl Instruction for I2F {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(val) => frame.operand_stack_push(Slot::Float(val as f32)),
            _ => panic!("i2x only support type Int"),
        };
    }
}

pub struct I2D {}

impl Instruction for I2D {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(val) => frame.operand_stack_push(Slot::Double(val as f64)),
            _ => panic!("i2x only support type Int"),
        };
    }
}


pub struct I2B {}

impl Instruction for I2B {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(val) => frame.operand_stack_push(Slot::Byte(val as u8)),
            _ => panic!("i2x only support type Int"),
        };
    }
}


pub struct I2C {}

impl Instruction for I2C {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(val) => frame.operand_stack_push(Slot::Char(val as u16)),
            _ => panic!("i2x only support type Int"),
        };
    }
}


pub struct I2S {}

impl Instruction for I2S {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(val) => frame.operand_stack_push(Slot::Short(val as i16)),
            _ => panic!("i2x only support type Int"),
        };
    }
}