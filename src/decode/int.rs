use crate::decode::{self, Cursor};

pub fn int(bytes: &mut Cursor) -> Option<i64> {
    match bytes.next()? {
        b'-' => minus(bytes),
        b'0' => zero(bytes),
        b => number(decode::ascii_number(b)?, bytes)
    }
}

fn minus(bytes: &mut Cursor) -> Option<i64> {
    match bytes.next()? {
        // i-0e not allowed
        b'0' => None,
        b => number(-decode::ascii_number(b)?, bytes)
    }
}

fn zero(bytes: &mut Cursor) -> Option<i64> {
    match bytes.next()? {
        b'e' => Some(0),
        _ => None
    }
}

fn number(val: i64, bytes: &mut Cursor) -> Option<i64> {
    match bytes.next()? {
        b'e' => Some(val),
        b => {
            let num = decode::ascii_number(b)?;
            let val = decode::push_digit_i(val, num)?;
            number(val, bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::cursor;

    #[test]
    fn works() {
        let mut i1 = cursor(b"42e");
        let mut i2 = cursor(b"-42e");
        let mut i3 = cursor(b"-4242e");

        let d1 = int(&mut i1);
        let d2 = int(&mut i2);
        let d3 = int(&mut i3);

        assert_eq!(d1, Some(42));
        assert_eq!(d2, Some(-42));
        assert_eq!(d3, Some(-4242));
    }

    #[test]
    fn int_leading_zero() {
        let mut i1 = cursor(b"042e");
        let mut i2 = cursor(b"-042e");
        let mut i3 = cursor(b"00e");
        let mut i4 = cursor(b"-00e");

        let d1 = int(&mut i1);
        let d2 = int(&mut i2);
        let d3 = int(&mut i3);
        let d4 = int(&mut i4);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
        assert_eq!(d3, None);
        assert_eq!(d4, None);
    }

    #[test]
    fn int_zero() {
        let mut i1 = cursor(b"0e");
        let mut i2 = cursor(b"-0e");

        let d1 = int(&mut i1);
        let d2 = int(&mut i2);

        assert_eq!(d1, Some(0));
        assert_eq!(d2, None);
    }

    #[test]
    fn empty() {
        let mut i1 = cursor(b"");
        let mut i2 = cursor(b"e");
        let mut i3 = cursor(b"-");
        let mut i4 = cursor(b"-e");

        let d1 = int(&mut i1);
        let d2 = int(&mut i2);
        let d3 = int(&mut i3);
        let d4 = int(&mut i4);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
        assert_eq!(d3, None);
        assert_eq!(d4, None);
    }

    #[test]
    fn no_delim() {
        let mut i1 = cursor(b"312");
        let mut i2 = cursor(b"-5121");
        let mut i3 = cursor(b"0");
        let mut i4 = cursor(b"000");

        let d1 = int(&mut i1);
        let d2 = int(&mut i2);
        let d3 = int(&mut i3);
        let d4 = int(&mut i4);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
        assert_eq!(d3, None);
        assert_eq!(d4, None);
    }

    #[test]
    fn non_digit() {
        let mut i1 = cursor(b"giowae");
        let mut i2 = cursor(b"je");
        let mut i3 = cursor(b"41904oifkwe");

        let d1 = int(&mut i1);
        let d2 = int(&mut i2);
        let d3 = int(&mut i3);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
        assert_eq!(d3, None);
    }

    #[test]
    fn too_big() {
        let mut i1 = cursor(b"49028104921402148214902149");

        let d1 = int(&mut i1);

        assert_eq!(d1, None);
    }
}
