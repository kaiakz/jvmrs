pub mod xadd;
pub mod xsub;
pub mod xmul;
pub mod xdiv;
pub mod xrem;
pub mod xneg;
pub mod sh;
pub mod bools;
pub mod iinc;

use crate::instructions::Instruction;

pub fn new(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x60 => Box::new(xadd::IADD {}),
        0x61 => Box::new(xadd::LADD {}),
        0x62 => Box::new(xadd::FADD {}),
        0x63 => Box::new(xadd::DADD {}),
        0x64 => Box::new(xsub::ISUB {}),
        0x65 => Box::new(xsub::LSUB {}),
        0x66 => Box::new(xsub::FSUB {}),
        0x67 => Box::new(xsub::DSUB {}),
        0x68 => Box::new(xmul::IMUL {}),
        0x69 => Box::new(xmul::LMUL {}),
        0x6a => Box::new(xmul::FMUL {}),
        0x6b => Box::new(xmul::DMUL {}),
        0x6c => Box::new(xdiv::IDIV {}),
        0x6d => Box::new(xdiv::LDIV {}),
        0x6e => Box::new(xdiv::FDIV {}),
        0x6f => Box::new(xdiv::DDIV {}), 
        0x70 => Box::new(xrem::IREM {}),
        0x71 => Box::new(xrem::LREM {}),
        0x72 => Box::new(xrem::FREM {}),
        0x73 => Box::new(xrem::DREM {}),
        0x74 => Box::new(xneg::INEG {}),
        0x75 => Box::new(xneg::LNEG {}),
        0x76 => Box::new(xneg::FNEG {}),
        0x77 => Box::new(xneg::DNEG {}),
        0x78 => Box::new(sh::ISH {left: true}),
        0x79 => Box::new(sh::LSH {left: true}),
        0x7a => Box::new(sh::ISH {left: false}),
        0x7b => Box::new(sh::LSH {left: false}),
        0x7c => Box::new(sh::IUSHR {}),
        0x7d => Box::new(sh::LUSHR {}),
        0x7e => Box::new(bools::IBOOL {bt: bools::BoolType::And}),
        0x7f => Box::new(bools::LBOOL {bt: bools::BoolType::And}), 
        0x80 => Box::new(bools::IBOOL {bt: bools::BoolType::Or}),
        0x81 => Box::new(bools::LBOOL {bt: bools::BoolType::Or}),
        0x82 => Box::new(bools::IBOOL {bt: bools::BoolType::Xor}),
        0x83 => Box::new(bools::LBOOL {bt: bools::BoolType::Xor}),
        0x84 => Box::new(iinc::IINC::new()),    
        _ => panic!("Instruction Error: math"),
    }
}
