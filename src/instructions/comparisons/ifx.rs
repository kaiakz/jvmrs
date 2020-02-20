use std::cmp::Ordering;
use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct IFEQ {
    offset: u32,
}

impl Instruction for IFEQ {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let value = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("ifx only supports type Int"),
        };

        if value == 0 {

        }
}

pub struct IFNE {
    offset: u32,
}

impl Instruction for IFNE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let value = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("ifx only supports type Int"),
        };

        if value != 0 {

        }
}

pub struct IFLT {
    offset: u32,
}

impl Instruction for IFLT {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let value = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("ifx only supports type Int"),
        };

        if value < 0 {

        }
}

pub struct IFLE {
    offset: u32,
}

impl Instruction for IFLE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let value = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("ifx only supports type Int"),
        };

        if value <= 0 {

        }
}

pub struct IFGT {
    offset: u32,
}

impl Instruction for IFGT {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let value = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("ifx only supports type Int"),
        };

        if value > 0 {

        }
}

pub struct IFGE {
    offset: u32,
}

impl Instruction for IFGE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {}

    fn excute(&self, frame: &mut Frame) {
        let value = match frame.operand_stack_pop() {
            Slot::Int(value) => value,
            _ => panic!("ifx only supports type Int"),
        };

        if value >= 0 {

        }
}
