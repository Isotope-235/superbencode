use std::io::Write;

pub fn int(val: i64, buf: &mut Vec<u8>) {
    write!(buf, "i{val}e").expect("Byte vec writes cannot fail");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let i1 = 42;
        let i2 = -42;

        let mut buf = Vec::new();
        int(i1, &mut buf);
        int(i2, &mut buf);

        assert_eq!(buf, b"i42ei-42e");
    }

    #[test]
    fn int_zero() {
        let i1 = 0;
        let i2 = 0;

        let mut buf = Vec::new();
        int(i1, &mut buf);
        int(i2, &mut buf);

        assert_eq!(buf, b"i0ei0e");
    }
}
