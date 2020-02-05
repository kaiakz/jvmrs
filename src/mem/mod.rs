pub mod thread;
pub mod frame;

#[derive(Clone)]
pub enum Slot {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    Float(f32),
    Double(f64),
    Ref,
}