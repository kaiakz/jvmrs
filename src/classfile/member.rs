use crate::utils::bytes::ByteStream;
use super::cp::ConstantPool;
use super::attribute::Attribute;

//Method && Field
pub struct Member {
    members: Vec<Info>,
}

impl Member {
    pub fn from(reader: &mut ByteStream, cp: &ConstantPool) -> Member {
        let count = reader.get_u16_be().expect("fields/methods_count") as usize;
        let mut v: Vec<Info> = Vec::with_capacity(count);

        for _i in 0..count {
            v.push(Info::from(reader, cp));
        }
        Member {
            members: v,
        }
    }
}

struct Info {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attribute: Attribute,    
}

impl Info {
    pub fn from(reader: &mut ByteStream, cp: &ConstantPool) -> Info {
        Info {
            access_flags: reader.get_u16_be().expect("access_flags"),
            name_index: reader.get_u16_be().expect("name_index"),
            descriptor_index: reader.get_u16_be().expect("descriptor_index"),
            attribute: Attribute::from(reader, cp),
        }
    }    
}