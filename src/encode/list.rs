use crate::Value;

pub fn list(vals: &[Value], buf: &mut Vec<u8>) {
    buf.push(b'l');
    for v in vals {
        super::inner(v, buf);
    }
    buf.push(b'e');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut buf = Vec::new();
        list(&[], &mut buf);
        assert_eq!(buf, b"le");
    }

    #[test]
    fn one_val() {
        let mut buf = Vec::new();
        let vals = vec![Value::Int(42)];

        list(&vals, &mut buf);

        assert_eq!(buf, b"li42ee");
    }

    #[test]
    fn preserves_buffer() {
        let mut buf = b"xyz".to_vec();
        let vals = vec![Value::Int(42)];

        list(&vals, &mut buf);

        assert_eq!(buf, b"xyzli42ee");
    }
}
