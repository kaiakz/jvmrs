use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;


pub struct ISTORE {
    index: usize,
    const_flag: bool,
}

impl ISTORE {
    pub fn init(index: usize, const_flag: bool) -> ISTORE {
        ISTORE {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Int(_val) => frame.local_variables_set(self.index, i),
            _ => panic!("Slot should be type Int"),
        };
    }
}

pub struct LSTORE {
    index: usize,
    const_flag: bool,
}

impl LSTORE {
    pub fn init(index: usize, const_flag: bool) -> LSTORE {
        LSTORE {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Long(_val) => frame.local_variables_set(self.index, i),
            _ => panic!("Slot should be type Long"),
        };
    }
}

pub struct FSTORE {
    index: usize,
    const_flag: bool,
}

impl FSTORE {
    pub fn init(index: usize, const_flag: bool) -> FSTORE {
        FSTORE {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Float(_val) => frame.local_variables_set(self.index, i),
            _ => panic!("Slot should be type Float"),
        };
    }
}

pub struct DSTORE {
    index: usize,
    const_flag: bool,
}

impl DSTORE {
    pub fn init(index: usize, const_flag: bool) -> DSTORE {
        DSTORE {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Double(_val) => frame.local_variables_set(self.index, i),
            _ => panic!("Slot should be type Double"),
        };
    }
}

pub struct ASTORE {
    index: usize,
    const_flag: bool,
}

impl ASTORE {
    pub fn init(index: usize, const_flag: bool) -> ASTORE {
        ASTORE {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.operand_stack_pop();
        match i {
            Slot::Ref => frame.local_variables_set(self.index, i),
            _ => panic!("Slot should be type Ref"),
        };
    }
}