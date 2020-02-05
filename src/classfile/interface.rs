use crate::utils::bytes::ByteStream;

pub struct Interface {
    interfaces : Vec<u16>
}

impl Interface {
    pub fn from(reader:&mut ByteStream) -> Interface {
        let interfaces_count = reader.get_u16_be().expect("Invaild interface_count") as usize;
        // println!("interfaces: {}", interfaces_count);
        let mut v : Vec<u16> = Vec::with_capacity(interfaces_count);
        // let v : Vec<u16> = Vec::new();

        for _i in 0..interfaces_count {
            v.push(reader.get_u16_be().expect("Read Interfaces Failed"));
        }
        // println!("interfaces: {:?}", &v);
        Interface {interfaces:v}
    }
}