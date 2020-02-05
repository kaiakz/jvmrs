pub mod cmd;
pub mod classpath;
pub mod classfile;
pub mod utils;
pub mod mem;
pub mod instructions;


// use self::cmd;
// use self::classpath;
use utils::bytes;

fn main() {
    let s = String::from("/run/media/kai/Data/Pro/rust/jvmrs/rt;/run/media/kai/Data/Pro/rust/jvmrs/classpath/target/debug/t.jar;/run/media/kai/Data/Pro/rust/jvmrs/classpath/target/debug/;/usr/lib/jvm/default-runtime/jre/lib/rt.jar;");
    let mut cp = classpath::ClassPath::new();
    cp.add(s);
    // cp.parse();
    let b = cp.fetch("java/lang/Class.class");
    let cf = classfile::ClassFile::load(b);
}

// fn test() {
//     let s = String::from("/run/media/kai/Data/Pro/rust/jvmrs/classpath/target/debug/t.jar;/run/media/kai/Data/Pro/rust/jvmrs/classpath/target/debug/;/usr/lib/jvm/default-runtime/jre/lib/rt.jar;");
//     let mut cp = classpath::ClassPath::new();
//     cp.add(s);
//     // cp.parse();
//     cp.fetch("java/lang/Object.class");
// }