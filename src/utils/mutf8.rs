use std::io::Cursor;
use std::io::Read;
use std::io;

pub fn decode(r:&mut Cursor<Vec<u8>>, length:usize) -> io::Result<String> {
    let mut buf:Vec<u8> = Vec::with_capacity(length);
    r.take(length as u64).read_to_end(&mut buf)?;
    let s = String::from_utf8_lossy(&buf);
    Ok(s.to_string())
}