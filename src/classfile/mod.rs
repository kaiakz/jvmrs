pub mod cp;
pub mod attribute;
pub mod member;
pub mod interface;

use crate::utils::bytes::ByteStream;
use crate::utils::bytes::Bytes;

//the data of classfiles store as big-endian
pub struct ClassFile {
    // magic : u32,
    minor_version : u16,
    majar_version : u16,
    constant_pool : cp::ConstantPool,              //use vector to store constantinfo and its len() to represent constantpool_count
    access_flags : u16,
    this_class : u16,
    super_class : u16,
    interfaces : interface::Interface,
    fields : member::Member,
    methods : member::Member,
    attributes : attribute::Attribute,
}

impl ClassFile {
    pub fn load(raw : Bytes) -> ClassFile {
        let mut reader = ByteStream::from(raw);
        // let mut reader = Cursor::new(code);
        // let magic = reader.get_u32_be();
        // let minorVersion = reader.get_u16_be();
        // let majarVersion = reader.get_u16_be();
        // let 
        // let accessFlags = reader.get_u16_be();
        // let thisClass = reader.get_u16_be();
        // let superClass = reader.get_u16_be();

        //Check Magic:0xCAFEBABE
        let magic = reader.get_u32_be().expect("Invaild magic");
        if magic != 0xCAFEBABE {
            panic!("Bad magic");
        }

        //Check Version
        let minor_version = reader.get_u16_be().expect("Invaild version");
        let majar_version = reader.get_u16_be().expect("Invaild version");

        // println!("minor_version:{}", &minor_version);
        // println!("majar_version:{}", &majar_version);

        //Read ConstantPool
        let cp = cp::ConstantPool::from(&mut reader);

        let access_flags = reader.get_u16_be().expect("Invaild access_flags");
        let this_class = reader.get_u16_be().expect("Invaild this_class");
        let super_class = reader.get_u16_be().expect("Invaild super_class");

        // println!("access_flags:{}", &access_flags);
        // println!("this_class:{}", &this_class);
        // println!("super_class:{}", &super_class);

        //Read interfaces
        let interfaces = interface::Interface::from(&mut reader);

        //Read Field
        let fields = member::Member::from(&mut reader, &cp);

        //Read Method
        let methods = member::Member::from(&mut reader, &cp);

        //Read Attributes
        let attributes = attribute::Attribute::from(&mut reader, &cp);

        ClassFile {
            // magic : reader.get_u32_be().expect("Invaild magic"),
            minor_version : minor_version,
            majar_version : majar_version,
            constant_pool : cp,
            access_flags : access_flags,
            this_class : this_class,
            super_class : super_class,
            interfaces : interfaces,
            fields : fields,
            methods : methods,
            attributes : attributes,
        }
    }
}