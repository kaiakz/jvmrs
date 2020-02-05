use crate::instructions::ByteCode;
use crate::instructions::Instruction;
use crate::mem::frame::Frame;
use crate::mem::Slot;

pub struct ILOAD {
    index: usize,
    const_flag: bool,
}

impl ILOAD {
    pub fn init(index: usize, const_flag: bool) -> ILOAD {
        ILOAD {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.local_variables_get(self.index);
        match i {
            Slot::Int(_val) => frame.operand_stack_push(i),
            _ => panic!("Slot should be type Int"),
        };
    }
}

pub struct LLOAD {
    index: usize,
    const_flag: bool,
}

impl LLOAD {
    pub fn init(index: usize, const_flag: bool) -> LLOAD {
        LLOAD {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for LLOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.local_variables_get(self.index);
        match i {
            Slot::Long(_val) => frame.operand_stack_push(i),
            _ => panic!("Slot should be type Long"),
        };
    }
}

pub struct FLOAD {
    index: usize,
    const_flag: bool,
}

impl FLOAD {
    pub fn init(index: usize, const_flag: bool) -> FLOAD {
        FLOAD {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.local_variables_get(self.index);
        match i {
            Slot::Float(_val) => frame.operand_stack_push(i),
            _ => panic!("Slot should be type Float"),
        };
    }
}

pub struct DLOAD {
    index: usize,
    const_flag: bool,
}

impl DLOAD {
    pub fn init(index: usize, const_flag: bool) -> DLOAD {
        DLOAD {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for DLOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.local_variables_get(self.index);
        match i {
            Slot::Double(_val) => frame.operand_stack_push(i),
            _ => panic!("Slot should be type Double"),
        };
    }
}

pub struct ALOAD {
    index: usize,
    const_flag: bool,
}

impl ALOAD {
    pub fn init(index: usize, const_flag: bool) -> ALOAD {
        ALOAD {
            index: index,
            const_flag: const_flag,
        }
    }
}

impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut ByteCode) {
        if !self.const_flag {
            self.index = reader.fetch_u8() as usize;
        }
    }

    fn excute(&self, frame: &mut Frame) {
        let i = frame.local_variables_get(self.index);
        match i {
            Slot::Ref => frame.operand_stack_push(i),
            _ => panic!("Slot should be type Ref"),
        };
    }
}