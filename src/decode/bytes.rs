use crate::decode::{self, Cursor};

pub fn bytes<'a>(data: &mut Cursor<'a>) -> Option<&'a [u8]> {
    let len = len(data)?;

    data.take(len)
}

fn len(data: &mut Cursor) -> Option<usize> {
    match data.next()? {
        b'0' => len_zero(data),
        b => len_number(decode::ascii_number(b)?, data)
    }
}

fn len_zero(data: &mut Cursor) -> Option<usize> {
    match data.next()? {
        b':' => Some(0),
        _ => None
    }
}

fn len_number(val: usize, data: &mut Cursor) -> Option<usize> {
    match data.next()? {
        b':' => Some(val),
        b => {
            let num = decode::ascii_number(b)?;
            let val = decode::push_digit_u(val, num)?;
            len_number(val, data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::cursor;

    #[test]
    fn works() {
        let mut i1 = cursor(b"10:0123456789");
        let mut i2 = cursor(b"1:e");

        let d1 = bytes(&mut i1);
        let d2 = bytes(&mut i2);

        assert_eq!(d1.unwrap(), b"0123456789");
        assert_eq!(d2.unwrap(), b"e");
    }

    #[test]
    fn nothing() {
        let mut i1 = cursor(b"0:");

        let d1 = bytes(&mut i1);

        assert_eq!(d1.unwrap(), b"");
    }

    #[test]
    fn moves_cursor() {
        let mut i1 = cursor(b"10:0123456789");
        let mut i2 = cursor(b"5:banani42e");

        let _d1 = bytes(&mut i1);
        let _d2 = bytes(&mut i2);

        assert_eq!(i1.bytes, b"");
        assert_eq!(i2.bytes, b"i42e");
    }

    #[test]
    fn bad_input() {
        let mut i1 = cursor(b"10:9");
        let mut i2 = cursor(b"0x0123456789");
        let mut i3 = cursor(b"4");
        let mut i4 = cursor(b"002:hi");

        let d1 = bytes(&mut i1);
        let d2 = bytes(&mut i2);
        let d3 = bytes(&mut i3);
        let d4 = bytes(&mut i4);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
        assert_eq!(d3, None);
        assert_eq!(d4, None);
    }
}
