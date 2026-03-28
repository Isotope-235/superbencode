use std::io::Write;

pub fn bytes(val: &[u8], buf: &mut Vec<u8>) {
    let len = val.len();
    write!(buf, "{len}:").expect("Byte vec writes cannot fail");
    buf.extend_from_slice(val);
}
