use std::collections::BTreeMap;

use crate::{
    Value,
    decode::{self, Cursor}
};

pub fn dict<'a>(bytes: &mut Cursor<'a>) -> Option<BTreeMap<&'a [u8], Value<'a>>> {
    let mut out = BTreeMap::new();

    let mut last = None;
    loop {
        match bytes.peek()? {
            b'e' => {
                bytes.skip();
                break Some(out);
            }
            b if b.is_ascii_digit() => {
                if let Some(e) = entry(bytes)
                    && last.is_none_or(|l| l < e.0)
                {
                    last = Some(e.0);
                    out.insert(e.0, e.1);
                } else {
                    break None;
                }
            }
            _ => break None
        }
    }
}

fn entry<'a>(rest: &mut Cursor<'a>) -> Option<(&'a [u8], Value<'a>)> {
    let key = decode::bytes(rest)?;
    let val = decode::inner(rest)?;
    Some((key, val))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::cursor;

    #[test]
    fn empty() {
        let mut i1 = cursor(b"e");
        let mut i2 = cursor(b"");

        let d1 = dict(&mut i1);
        let d2 = dict(&mut i2);

        assert!(d1.unwrap().is_empty());
        assert_eq!(d2, None);
    }

    #[test]
    fn one_entry() {
        let mut cursor = cursor(b"3:key5:valuee");
        let result = dict(&mut cursor);
        assert_eq!(
            result.unwrap(),
            BTreeMap::from([(b"key".as_slice(), Value::Bytes(b"value"))])
        );
    }

    #[test]
    fn rejects_nonsense() {
        let mut cursor = cursor(b"d:kdw::dwad1kwoapf12:keykeykeykey5:banane");
        let result = dict(&mut cursor);
        assert_eq!(result, None);
    }

    #[test]
    fn bad_order() {
        let mut cursor = cursor(b"5:applei2e5:Zebrai1e5:banani-1ee");
        let result = dict(&mut cursor);
        assert_eq!(result, None);
    }

    #[test]
    fn key_bad_val() {
        let mut i1 = cursor(b"5:worms");
        let mut i2 = cursor(b"5:wormse");
        let mut i3 = cursor(b"5:worms6:e");
        let mut i4 = cursor(b"5:worms6:");
        let mut i5 = cursor(b"5:worms6");

        let d1 = dict(&mut i1);
        let d2 = dict(&mut i2);
        let d3 = dict(&mut i3);
        let d4 = dict(&mut i4);
        let d5 = dict(&mut i5);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
        assert_eq!(d3, None);
        assert_eq!(d4, None);
        assert_eq!(d5, None);
    }

    #[test]
    fn bad_key() {
        let mut i1 = cursor(b"5app3:vale");
        let mut i2 = cursor(b"5");

        let d1 = dict(&mut i1);
        let d2 = dict(&mut i2);

        assert_eq!(d1, None);
        assert_eq!(d2, None);
    }
}
