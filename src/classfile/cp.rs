use crate::utils::bytes::ByteStream;
use crate::utils::mutf8;

pub struct ConstantPool {
    cp : Vec<Info>
}

impl ConstantPool {
    
    pub fn from(reader:&mut ByteStream) -> ConstantPool {
        let constant_pool_count = reader.get_u16_be().expect("Invalid constant_pool_count");
        // let constant_pool_len : usize = (constant_pool_count - 1).try_into().expect("Fail Convert:u32 to usize");
        let constant_pool_len = (constant_pool_count - 1) as usize;
        let mut cp = Vec::with_capacity(constant_pool_len);
        // let cp : Vec<Info> = Vec::new();
        // println!("cp_len is {}", &constant_pool_len);

        let mut i = 1;
        cp.push(Info::Nothing); //ConstantPool's 0 index is invaild

        while i <= constant_pool_len {
            let tag = reader.get_u8().expect("Read Tag Failed");
            match Tag::from(tag) {

                Tag::Utf8 => {
                    let length = reader.get_u16_be().expect("Error Tag") as usize;
                    // print!("length {}", &length);
                    let s = mutf8::decode(&mut reader.as_cursor(), length).expect("Error Tag: Utf8");
                    // print!("({})", i);
                    // println!("utf8: {}", &s);
                    cp.push(Info::Utf8{val : s});
                },

                Tag::Integer => {
                    cp.push(Info::Integer{val:reader.get_i32_be().expect("Error Tag: Integer")});
                },

                Tag::Float => {
                    cp.push(Info::Float{val : reader.get_f32_be().expect("Error Tag: Float")});
                },

                Tag::Long => {
                    cp.push(Info::Long{val : reader.get_i64_be().expect("Error Tag")});
                    cp.push(Info::Nothing);
                    i += 1;
                },

                Tag::Double => {
                    cp.push(Info::Double{val : reader.get_f64_be().expect("Error Tag")});
                    cp.push(Info::Nothing);
                    i += 1;
                },

                Tag::Class => {
                    cp.push(Info::Class{name_index : reader.get_u16_be().expect("Error Tag")});
                },

                Tag::String => {
                    cp.push(Info::String{utf8_index : reader.get_u16_be().expect("Error Tag")});
                },

                Tag::Fieldref => {
                    cp.push(Info::Fieldref{
                        class_index : reader.get_u16_be().expect("Error Tag:"),
                        name_and_type_index : reader.get_u16_be().expect("Error Tag:"),
                    });
                },

                Tag::Methodref => {
                    cp.push(Info::Methodref{
                        class_index : reader.get_u16_be().expect("Error Tag:"),
                        name_and_type_index : reader.get_u16_be().expect("Error Tag:"),
                    });
                },

                Tag::InterfaceMethodref => {
                    cp.push(Info::InterfaceMethodref{
                        class_index : reader.get_u16_be().expect("Error Tag:"),
                        name_and_type_index : reader.get_u16_be().expect("Error Tag:"),                        
                    });
                },

                Tag::NameAndType => {
                    cp.push(Info::NameAndType{
                        name_index : reader.get_u16_be().expect("Error Tag:"),
                        descriptor_index : reader.get_u16_be().expect("Error Tag:"),
                    });
                },

                Tag::MethodHandle => {
                    cp.push(Info::MethodHandle{
                        reference_kind : reader.get_u8().expect("Error Tag:"),
                        reference_index : reader.get_u16_be().expect("Error Tag:"),
                    });
                },

                Tag::MethodType => {
                    cp.push(Info::MethodType{
                        descriptor_index : reader.get_u16_be().expect("Error Tag:"),
                    });
                },

                Tag::InvokeDynamic => {
                    cp.push(Info::InvokeDynamic{
                        bootstrap_method_attr_index : reader.get_u16_be().expect("Error Tag:"),
                        name_and_type_index : reader.get_u16_be().expect("Error Tag:"),                        
                    });
                },
                
                // _ => {
                //     println!("{}", tag);
                //     panic!("java.lang.ClassFormatError : Unsupported ConstantPool Info");
                // },
            }
            i += 1;
        }
        // println!("cp_actual_len: {}", cp.len());
        ConstantPool {cp : cp}
    }

    pub fn get_constantpool_count(&self) -> usize {
        self.cp.len() + 1
    }

    // pub fn get_constantpool_by_index(&self, cp_index:usize) -> &Info {
    //     self.cp.get(cp_index).expect("Invalid index")
    // }

    pub fn get_utf8(&self, cp_index:usize) -> Option<&String> {
        // print!("index {}  ", cp_index);
        let c = self.cp.get(cp_index).expect("Invalid cp_index");
        match c {
            Info::Utf8{val} => {
                // println!("get: {}", &val);
                Some(val)
            },
            Info::Nothing => None,
            _ => panic!("Error Info: Not Utf8"),
        }
    }
}

enum Tag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
}

impl Tag {
    pub fn from(i:u8) -> Tag {
        match i {
            1 => Tag::Utf8,
            3 => Tag::Integer,
            4 => Tag::Float,
            5 => Tag::Long,
            6 => Tag::Double,
            7 => Tag::Class,
            8 => Tag::String,
            9 => Tag::Fieldref,
            10 => Tag::Methodref,
            11 => Tag::InterfaceMethodref,
            12 => Tag::NameAndType,
            15 => Tag::MethodHandle,
            16 => Tag::MethodType,
            18 => Tag::InvokeDynamic,
            _ => {
                println!("panic tag {}", i);
                panic!("java.lang.ClassFormatError : ConstantPool Tag") 
            }
        }
    } 
}

enum Info {
    Utf8 {val:String},
    Integer {val:i32},
    Float {val:f32},
    Long {val:i64},
    Double {val:f64},
    Class {name_index:u16},
    String {utf8_index:u16},
    Fieldref {class_index:u16, name_and_type_index:u16},
    Methodref {class_index:u16, name_and_type_index:u16},
    InterfaceMethodref {class_index:u16, name_and_type_index:u16},
    NameAndType {name_index:u16, descriptor_index:u16},
    MethodHandle {reference_kind:u8, reference_index:u16},
    MethodType {descriptor_index:u16},
    InvokeDynamic {bootstrap_method_attr_index:u16, name_and_type_index:u16},
    Nothing,
}



