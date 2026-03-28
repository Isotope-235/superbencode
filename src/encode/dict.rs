use std::collections::BTreeMap;

use crate::{Value, encode};

pub fn dict<'a>(entries: &BTreeMap<&'a [u8], Value<'a>>, buf: &mut Vec<u8>) {
    buf.push(b'd');
    for (k, v) in entries {
        encode::bytes(k, buf);
        encode::inner(v, buf);
    }
    buf.push(b'e');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn empty() {
        let mut buf = Vec::new();
        dict(&BTreeMap::new(), &mut buf);
        assert_eq!(buf, b"de");
    }

    #[test]
    fn one_entry() {
        let mut buf = Vec::new();
        let entries = BTreeMap::from([(b"key".as_slice(), Value::Int(42))]);

        dict(&entries, &mut buf);

        assert_eq!(buf, b"d3:keyi42ee");
    }

    #[test]
    fn multiple_entries() {
        let mut buf = Vec::new();
        let entries = BTreeMap::from([
            (b"a-key".as_slice(), Value::Int(1)),
            (b"".as_slice(), Value::List(Vec::new()))
        ]);

        dict(&entries, &mut buf);

        assert_eq!(buf, b"d0:le5:a-keyi1ee");
    }
}
