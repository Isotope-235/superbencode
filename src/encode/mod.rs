mod bytes;
mod dict;
mod int;
mod list;

pub use bytes::*;
pub use dict::*;
pub use int::*;
pub use list::*;

use crate::Value;

pub fn inner(val: &Value, buf: &mut Vec<u8>) {
    match *val {
        Value::Bytes(items) => self::bytes(items, buf),
        Value::Int(val) => self::int(val, buf),
        Value::List(ref vals) => self::list(vals, buf),
        Value::Dict(ref entries) => self::dict(entries, buf)
    }
}
