use crate::utils::bytes::ByteStream;
use crate::utils::bytes::Bytes;
use super::cp;

pub struct Attribute {
    info : Vec<Info>,
}

// enum Name {
//     Code,
//     ConstantValue,
//     Deprecated,
//     Exceptions,
//     LineNumberTable,
//     LocalVaribleTable,
//     SourceFile,
//     Synthetic,
// }

enum Info {
    Code {
        max_stack:u16,
        max_locals:u16,
        //code_length
        code:Bytes,
        //exception_table_length
        exception_table: ExceptionTable,
        //attribute_count
        attribute_info: Attribute,
    },

    ConstantValue {constantvalue_index:u16},

    Deprecated,

    Exceptions {
        //number_of_exceptions: u16
        exception_index_table: Vec<u16>,
    },
    
    LineNumberTable {
        line_number_table: LineNumberTable,
    },
    
    LocalVaribleTable {
        local_variable_table: LocalVariableTable,
    },
    
    SourceFile {sourfile_index:u16},
    
    Synthetic,
    
    _Unparsed {name:String, info:Bytes},    
}

impl Info {

    pub fn from(reader:&mut ByteStream, cp: &cp::ConstantPool) -> Info {
        let name_index = reader.get_u16_be().expect("Invaild name_index") as usize;
        let name = cp.get_utf8(name_index);
        let length = reader.get_u32_be().expect("Invaild attribute_length") as usize;

        let s = match name {
            Some(s) => s.as_str(),
            None => "Unparsed",
        };
        
        // print!("Got Index {}", s);
        
        match s {

            "Code" => {
                let max_stack = reader.get_u16_be().expect("max_stack");
                let max_locals = reader.get_u16_be().expect("max_locals");
                let code_length = reader.get_u32_be().expect("code_length") as usize;
                let code = reader.get_bytes(code_length).expect("code");                
                let exception_table = ExceptionTable::from(reader);
                let attribute_info = Attribute::from(reader, cp);
                
                // println!("max_stack {}", &max_stack);
                // println!("max_locals {}", &max_locals);
                // println!("code_length {}", &code_length);
                // println!("code {:?}", &code);

                Info::Code {
                    max_stack: max_stack,
                    max_locals: max_locals,
                    code: code,
                    exception_table: exception_table,
                    attribute_info: attribute_info,
                }
            },

            "ConstantValue" => {
                Info::ConstantValue {
                    constantvalue_index : reader.get_u16_be().expect("constantvalue_index"),
                }
            },

            "Deprecated" => {Info::Deprecated},

            "Exceptions" => {
                let number_of_exceptions = reader.get_u16_be().expect("number_of_exceptions") as usize;
                let mut v : Vec<u16> = Vec::with_capacity(number_of_exceptions);

                for _i in 0..number_of_exceptions {
                    v.push(reader.get_u16_be().expect("exception_index_table"));
                }

                Info::Exceptions {
                    exception_index_table: v,
                }
            },

            "LineNumberTable" => {
                Info::LineNumberTable {
                    line_number_table: LineNumberTable::from(reader),
                }
            },
            
            "LocalVaribleTable" => {
                Info::LocalVaribleTable {
                    local_variable_table: LocalVariableTable::from(reader),
                }
            },

            "SourceFile" => {
                Info::SourceFile {
                    sourfile_index : reader.get_u16_be().expect("Invaild Attribute : SourceFile"),
                }
            },

            "Synthetic" => {Info::Synthetic},

            _ => {
                // println!("Unparsed: {}", s);
                Info::_Unparsed {
                    name : s.to_string(),
                    info : reader.get_bytes(length).expect("Invaild attribute_info"),
                }
            },
        }
    }
}

impl Attribute {
    pub fn from(reader: &mut ByteStream, cp: &cp::ConstantPool) -> Attribute {
        let attributes_count = reader.get_u16_be().expect("Invaild attribute_count") as usize;
        let mut v : Vec<Info> = Vec::with_capacity(attributes_count);
        // println!("attributes_count {}", &attributes_count);
        for _i in 0..attributes_count {
            let info = Info::from(reader, cp);
            v.push(info);
        }
        Attribute {info:v}
    }
}



struct ExceptionTable {
    exception_table: Vec<ExcetionTableEntry>,
}

struct ExcetionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTable {
    pub fn from(reader: &mut ByteStream) -> ExceptionTable {
        let exception_table_length = reader.get_u16_be().expect("exception_table_length");
        let mut v : Vec<ExcetionTableEntry> = Vec::with_capacity(exception_table_length as usize);
        // println!("etlen {}", &exception_table_length);
        for _i in 0..exception_table_length {
            v.push(ExcetionTableEntry {
                start_pc: reader.get_u16_be().expect("start_pc"),
                end_pc: reader.get_u16_be().expect("end_pc"),
                handler_pc: reader.get_u16_be().expect("handler_pc"),
                catch_type: reader.get_u16_be().expect("catch_type"),
            });
        }
        ExceptionTable {exception_table: v}
    }
}



struct LineNumberTable {
    line_number_table: Vec<LineNumberTableEntry>
}

struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberTable {
    pub fn from(reader: &mut ByteStream) -> LineNumberTable {
        let line_number_table_length = reader.get_u16_be().expect("line_number_table_length");
        let mut v : Vec<LineNumberTableEntry> = Vec::with_capacity(line_number_table_length as usize);
        for _i in 0..line_number_table_length {
            v.push(LineNumberTableEntry {
                start_pc: reader.get_u16_be().expect("start_pc"),
                line_number: reader.get_u16_be().expect("line_number"),
            });
        }
        LineNumberTable {line_number_table: v}
    }
}



struct LocalVariableTable {
    local_variable_table: Vec<LocalVariableTableEntry>
}

struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariableTable {
    pub fn from(reader: &mut ByteStream) -> LocalVariableTable {
        let local_variable_table_table_length = reader.get_u16_be().expect("local_variable_table_table_length");
        let mut v : Vec<LocalVariableTableEntry> = Vec::with_capacity(local_variable_table_table_length as usize);
        for _i in 0..local_variable_table_table_length {
            v.push(LocalVariableTableEntry {
                start_pc: reader.get_u16_be().expect("start_pc"),
                length: reader.get_u16_be().expect("length"),
                name_index: reader.get_u16_be().expect("name_index"),
                descriptor_index: reader.get_u16_be().expect("descriptor_index"),
                index: reader.get_u16_be().expect("index"),
            });
        }
        LocalVariableTable {local_variable_table: v}
    }
}