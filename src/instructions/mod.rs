pub mod constants;
pub mod loads;
pub mod stores;
pub mod stack;
pub mod math;
pub mod conversions;

use crate::utils::bytes;
use crate::utils::bytes::Bytes;
use crate::mem::frame::Frame;

pub struct ByteCode {
    code: Bytes,
    pc: usize,
}

impl ByteCode {

    pub fn from(bytes: &Bytes) -> ByteCode {
        ByteCode {
            code: bytes.clone(),
            pc: 0,
        }
    }

    pub fn reset(&mut self, bytes: &Bytes, pc: usize) {
        self.code = bytes.clone();
        self.pc = pc;
    }

    pub fn fetch_u8(&mut self) -> u8 {
        let i = bytes::get_u8(&self.code, self.pc);
        self.pc += 1;
        i
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let i = bytes::get_u16_be(&self.code, self.pc);
        self.pc += 2;
        i 
    }

    pub fn fetch_u32(&mut self) -> u32 {
        let i = bytes::get_u32_be(&self.code, self.pc);
        self.pc += 4;
        i
    }

    pub fn fetch_i32(&mut self) -> i32 {
        let i = bytes::get_i32_be(&self.code, self.pc);
        self.pc += 4;
        i
    }

    pub fn fetch_i64(&mut self) -> i64 {
        let i = bytes::get_i64_be(&self.code, self.pc);
        self.pc += 8;
        i
    }

    pub fn fetch_f32(&mut self) -> f32 {
        let i = bytes::get_f32_be(&self.code, self.pc);
        self.pc += 4;
        i
    }

    pub fn fetch_f64(&mut self) -> f64 {
        let i = bytes::get_f64_be(&self.code, self.pc);
        self.pc += 8;
        i
    }
}

pub trait Instruction {
    fn fetch_operands(&mut self, reader: &mut ByteCode);
    fn excute(&self, frame: &mut Frame);
}




// struct NoOperandsInsruction {}

// impl Instruction for NoOperandsInsruction {
//     fn fetch_operands(&mut self, r: &mut ByteCode) {}
//     fn excute(&self) {}
// }

// struct BranchInstruction {
//     offset: usize,
// }

// impl Instruction for BranchInstruction {
//     fn fetch_operands(&mut self, r: &mut ByteCode) {
//         self.offset = r.fetch_u16() as usize;
//     }
//     fn excute(&self) {

//     }
// }

// struct Index8Instruction {
//     index: usize,
// }

// impl Instruction for Index8Instruction {
//     fn fetch_operands(&mut self, r: &mut ByteCode) {
//         self.index = r.fetch_u8() as usize;
//     }

//     fn excute(&self) {
        
//     }
// }

// struct Index16Instruction {
//     index: usize,
// }

// impl Instruction for Index16Instruction {
//     fn fetch_operands(&mut self, r: &mut ByteCode) {
//         self.index = r.fetch_u16() as usize;
//     }
    
//     fn excute(&self) {
        
//     }
// }

// fn dispatch(f: &mut Frame) {
//     let opcode: u8 = 0x01;
//     let inst = match opcode {
//         0x00..=0x0f => constants::xconst::new(opcode),
//         0x02 => constants::ipush::new(opcode),
//         _=> ,
//     };
//     inst.excute(f);
// }