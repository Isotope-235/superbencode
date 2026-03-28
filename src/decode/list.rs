use crate::{
    Value,
    decode::{self, Cursor}
};

pub fn list<'a>(bytes: &mut Cursor<'a>) -> Option<Vec<Value<'a>>> {
    let mut out = Vec::new();

    loop {
        match bytes.peek()? {
            b'e' => {
                bytes.skip();
                break Some(out);
            }
            _ => out.push(decode::inner(bytes)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::cursor;

    #[test]
    fn empty_list() {
        let mut i1 = cursor(b"e");
        let d1 = list(&mut i1);
        assert!(d1.is_some_and(|d| d.is_empty()));
    }

    #[test]
    fn one_val() {
        let mut i1 = cursor(b"i42ee");
        let mut i2 = cursor(b"4:bruhi3e5:banane");

        let d1 = list(&mut i1);
        let d2 = list(&mut i2);

        assert!(d1.is_some_and(|d| !d.is_empty()));
        assert_eq!(d2.unwrap(), [
            Value::Bytes(b"bruh"),
            Value::Int(3),
            Value::Bytes(b"banan")
        ]);

        assert!(i1.bytes.is_empty(), "Should advance cursor correctly");
        assert!(i1.bytes.is_empty(), "Should advance cursor correctly");
    }

    #[test]
    fn moves_cursor() {
        let mut i1 = cursor(b"i42ee5:banan");
        let _d1 = list(&mut i1);
        assert_eq!(i1.bytes, b"5:banan", "Cursor should be at end of list");
    }

    #[test]
    fn no_delimiter() {
        let mut i1 = cursor(b"i42e");
        let d1 = list(&mut i1);
        assert_eq!(d1, None);
    }

    #[test]
    fn invalid_byte() {
        let mut i1 = cursor(b"xi42ee");
        let d1 = list(&mut i1);
        assert_eq!(d1, None);
    }

    #[test]
    fn empty_input() {
        let mut i1 = cursor(b"");
        let d1 = list(&mut i1);
        assert_eq!(d1, None);
    }
}
