mod bytes;
mod dict;
mod int;
mod list;

pub use bytes::*;
pub use dict::*;
pub use int::*;
pub use list::*;

use crate::Value;

/// See [`Cursor`].
pub fn cursor(bytes: &[u8]) -> Cursor<'_> {
    Cursor { bytes }
}

/// Basically just a slice iterator (`core::slice::iter`) but it returns `u8` instead of `&u8`.
#[derive(Debug)]
pub struct Cursor<'buf> {
    pub bytes: &'buf [u8]
}

impl<'buf> Cursor<'buf> {
    /// Gets the next byte while advancing the cursor.
    pub fn next(&mut self) -> Option<u8> {
        let (n, rest) = self.bytes.split_first()?;
        self.bytes = rest;
        Some(*n)
    }

    pub fn peek(&self) -> Option<u8> {
        self.bytes.first().copied()
    }

    pub fn skip(&mut self) {
        let _ = self.next();
    }

    fn skip_to_end(&mut self) {
        self.bytes = &self.bytes[self.bytes.len()..];
    }

    pub fn take<'a>(&'a mut self, len: usize) -> Option<&'buf [u8]> {
        if let Some((begin, end)) = self.bytes.split_at_checked(len) {
            self.bytes = end;
            Some(begin)
        } else {
            self.skip_to_end();
            None
        }
    }
}

pub fn inner<'a>(bytes: &mut Cursor<'a>) -> Option<Value<'a>> {
    // the bytestring parser needs a contigouos slice to efficiently parse the string length
    // so `peek` is called instead of `next`, in order too keep the byte in the slice
    if bytes.peek()?.is_ascii_digit() {
        self::bytes(bytes).map(Value::Bytes)
    } else {
        match bytes.next().expect("Known to be present at this point") {
            b'i' => self::int(bytes).map(Value::Int),
            b'l' => self::list(bytes).map(Value::List),
            b'd' => self::dict(bytes).map(Value::Dict),
            _ => None
        }
    }
}

fn push_digit_i(val: i64, digit: i64) -> Option<i64> {
    let sign = val.signum();
    // push the digit:
    //   50 push 3 ->  503
    //  -43 push 5 -> -435
    val.checked_mul(10)?.checked_add(sign * digit)
}

fn push_digit_u(val: usize, digit: usize) -> Option<usize> {
    val.checked_mul(10)?.checked_add(digit)
}

fn ascii_number<T: From<u8>>(b: u8) -> Option<T> {
    if b.is_ascii_digit() {
        Some(T::from(b - b'0'))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_number_works() {
        assert_eq!(ascii_number(b'0'), Some(0));
        assert_eq!(ascii_number(b'1'), Some(1));
        assert_eq!(ascii_number(b'2'), Some(2));
        assert_eq!(ascii_number(b'3'), Some(3));
        assert_eq!(ascii_number(b'4'), Some(4));
        assert_eq!(ascii_number(b'5'), Some(5));
        assert_eq!(ascii_number(b'6'), Some(6));
        assert_eq!(ascii_number(b'7'), Some(7));
        assert_eq!(ascii_number(b'8'), Some(8));
        assert_eq!(ascii_number(b'9'), Some(9));
    }

    #[test]
    fn ascii_digit_negative() {
        assert_eq!(ascii_number::<i32>(b'a'), None);
        assert_eq!(ascii_number::<i32>(b'z'), None);
        assert_eq!(ascii_number::<i32>(b' '), None);
    }
}
