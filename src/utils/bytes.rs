use std::io::Read;
use std::io::Cursor;
use std::io;
use std::convert::TryInto;

pub type Bytes = Vec<u8>;

pub struct ByteStream {
    bytes: Cursor<Bytes>,
}

impl ByteStream {
    pub fn from(bytes:Bytes) -> ByteStream {
        ByteStream {
            bytes: Cursor::new(bytes),
        }
    }

    pub fn load(&mut self, bytes: Bytes) {
        self.bytes = Cursor::new(bytes);
    }

    pub fn as_cursor(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.bytes
    }

    pub fn get_u8(&mut self) -> io::Result<u8> {
        let mut b:[u8;1] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(u8::from_be_bytes(b))
    }

    pub fn get_u16_be(&mut self) -> io::Result<u16> {
        let mut b:[u8;2] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(u16::from_be_bytes(b))
    }

    pub fn get_u32_be(&mut self) -> io::Result<u32> {
        let mut b:[u8;4] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(u32::from_be_bytes(b))
    }

    pub fn get_u64_be(&mut self) -> io::Result<u64> {
        let mut b:[u8;8] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(u64::from_be_bytes(b))
    }

    pub fn get_i32_be(&mut self) -> io::Result<i32> {
        let mut b:[u8;4] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(i32::from_be_bytes(b))
    }

    pub fn get_i64_be(&mut self) -> io::Result<i64> {
        let mut b:[u8;8] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(i64::from_be_bytes(b))
    }

    pub fn get_f32_be(&mut self) -> io::Result<f32> {
        let mut b:[u8;4] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(f32::from_be_bytes(b))
    }

    pub fn get_f64_be(&mut self) -> io::Result<f64> {
        let mut b:[u8;8] = Default::default();
        self.bytes.read_exact(&mut b)?;
        Ok(f64::from_be_bytes(b))
    }

    pub fn get_bytes(&mut self, length:usize) -> io::Result<Vec<u8>> {
        // // println!("======Start get_bytes {}", &length);
        let mut buf:Vec<u8> = Vec::with_capacity(length);
        // println!("get {} bytes", length as u64);
        // self.bytes.get_ref().take(length as u64).read_to_end(&mut buf)?;
        // // println!("======Finish get_bytes {:?}", &buf);
        // Ok(buf)
        // let mut b:[u8;1] = Default::default();
        for _i in 0..length {
            // self.bytes.read_exact(&mut b)?;
            // let m : Vec<[u8;1]> = Vec::new();
            // println!("a byte {:?}", &b);
            buf.push(self.get_u8()?);
        }
        Ok(buf)
    }

    // pub fn get<T>(&mut self) -> T {
    //     // let size = ;
    //     let a : [T;std::mem::size_of::<T>()] = Default::default();
    //     self.bytes.read_exact(&mut a)?;

    // }
}

// pub struct ByteRaw {
//     bytes: Bytes,
//     position: usize,
// }

// impl ByteRaw {
//     pub fn from(v: Vec<u8>) -> ByteRaw {
//         ByteRaw {
//             bytes: v,
//             position: 0,
//         }
//     }

//     pub fn read<T>(&mut self) -> T {
//         let len = std::mem::size_of::<T>();
//         let result = 0;
        
//     }
// }

// #[allow(dead_code)]
// pub fn get_u8(r:&mut Cursor<Vec<u8>>) -> io::Result<u8> {
//     let mut b:[u8;1] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(u8::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_u16_be(r:&mut Cursor<Vec<u8>>) -> io::Result<u16> {
//     let mut b:[u8;2] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(u16::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_u32_be(r:&mut Cursor<Vec<u8>>) -> io::Result<u32> {
//     let mut b:[u8;4] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(u32::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_u64_be(r:&mut Cursor<Vec<u8>>) -> io::Result<u64> {
//     let mut b:[u8;8] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(u64::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_i32_be(r:&mut Cursor<Vec<u8>>) -> io::Result<i32> {
//     let mut b:[u8;4] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(i32::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_i64_be(r:&mut Cursor<Vec<u8>>) -> io::Result<i64> {
//     let mut b:[u8;8] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(i64::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_f32_be(r:&mut Cursor<Vec<u8>>) -> io::Result<f32> {
//     let mut b:[u8;4] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(f32::from_be_bytes(b))
// }

// #[allow(dead_code)]
// pub fn get_f64_be(r:&mut Cursor<Vec<u8>>) -> io::Result<f64> {
//     let mut b:[u8;8] = Default::default();
//     r.read_exact(&mut b)?;
//     Ok(f64::from_be_bytes(b))
// }



// #[allow(dead_code)]
pub fn get_u8(bytes: &Bytes, pos: usize) -> u8 {
    let b:&[u8; 1] = match bytes.get(pos..pos+1) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    u8::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_u16_be(bytes: &Bytes, pos: usize) -> u16 {
    let b:&[u8; 2] = match bytes.get(pos..pos+2) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    u16::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_u32_be(bytes: &Bytes, pos: usize) -> u32 {
    let b:&[u8; 4] = match bytes.get(pos..pos+4) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    u32::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_u64_be(bytes: &Bytes, pos: usize) -> u64 {
    let b:&[u8; 8] = match bytes.get(pos..pos+8) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    u64::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_i32_be(bytes: &Bytes, pos: usize) -> i32 {
    let b:&[u8; 4] = match bytes.get(pos..pos+4) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    i32::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_i64_be(bytes: &Bytes, pos: usize) -> i64 {
    let b:&[u8; 8] = match bytes.get(pos..pos+8) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    i64::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_f32_be(bytes: &Bytes, pos: usize) -> f32 {
    let b:&[u8; 4] = match bytes.get(pos..pos+4) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    f32::from_be_bytes(*b)
}

// #[allow(dead_code)]
pub fn get_f64_be(bytes: &Bytes, pos: usize) -> f64 {
    let b:&[u8; 8] = match bytes.get(pos..pos+8) {
        Some(s) => s.try_into().expect("slice with incorrect length"),
        None => panic!("out of bounds"),
    };
    f64::from_be_bytes(*b)
}