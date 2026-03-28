use crate::decode::cursor;

mod decode;
mod encode;
mod value;
pub use value::Value;

#[must_use]
pub fn decode(bytes: &[u8]) -> Option<Value<'_>> {
    let mut cursor = cursor(bytes);

    decode::inner(&mut cursor)
}

pub fn encode(val: &Value, buf: &mut Vec<u8>) {
    encode::inner(val, buf);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn make() {
        let buf1 = b"4:1234".to_vec().into_boxed_slice();
        {
            let buf2 = b"6:123456".to_vec().into_boxed_slice();
            let _val = Value::List(vec![Value::Bytes(&buf1), Value::Bytes(&buf2)]);
        }
    }

    #[test]
    fn file() {
        let input = include_bytes!("../test/bbb_sunflower.torrent");
        let decoded = decode(input).unwrap();
        let mut encoded = Vec::new();
        encode(&decoded, &mut encoded);
        assert_eq!(encoded, input);
    }

    #[test]
    fn invertible() {
        let i1 = Value::Bytes(b"ieh");
        let i2 = b"5:seven";

        let mut e1 = Vec::new();
        encode(&i1, &mut e1);
        let d2 = decode(i2).unwrap();

        let d1 = decode(&e1).unwrap();
        let mut e2 = Vec::new();
        encode(&d2, &mut e2);

        assert_eq!(i1, d1);
        assert_eq!(&i2[..], &e2);
    }

    mod decode {
        use super::*;

        #[test]
        fn comprehensive() {
            let input = b"d4:dictd3:key5:valuee4:listli42e5:helloee";

            let decoded = decode(input);

            assert_eq!(
                decoded.unwrap(),
                Value::Dict(BTreeMap::from([
                    (
                        b"dict".as_slice(),
                        Value::Dict(BTreeMap::from([(
                            b"key".as_slice(),
                            Value::Bytes(b"value")
                        )]))
                    ),
                    (
                        b"list".as_slice(),
                        Value::List(vec![Value::Int(42), Value::Bytes(b"hello")])
                    )
                ]))
            );
        }

        #[test]
        fn bytes() {
            let input = b"5:banan";

            let decoded = decode(input);

            assert_eq!(decoded.unwrap(), Value::Bytes(b"banan"));
        }

        #[test]
        fn int() {
            let i1 = b"i42e";
            let i2 = b"i-42e";

            let d1 = decode(i1);
            let d2 = decode(i2);

            assert_eq!(d1.unwrap(), Value::Int(42));
            assert_eq!(d2.unwrap(), Value::Int(-42));
        }

        #[test]
        fn empty() {
            let i1 = b"";
            let d1 = decode(i1);
            assert_eq!(d1, None);
        }

        mod int {
            use super::*;

            #[test]
            fn leading_zero() {
                let i1 = b"i042e";
                let i2 = b"i-042e";
                let i3 = b"i00e";

                let d1 = decode(i1);
                let d2 = decode(i2);
                let d3 = decode(i3);

                assert_eq!(d1, None);
                assert_eq!(d2, None);
                assert_eq!(d3, None);
            }

            #[test]
            fn zero() {
                let i1 = b"i0e";
                let i2 = b"i-0e";

                let d1 = decode(i1);
                let d2 = decode(i2);

                assert_eq!(d1.unwrap(), Value::Int(0));
                assert_eq!(d2, None);
            }
        }
    }

    mod encode {
        use super::*;

        #[test]
        fn dict() {
            let mut map = BTreeMap::new();
            let mut inner_map = BTreeMap::new();
            inner_map.insert(b"key".as_slice(), Value::Bytes(b"value"));

            map.insert(b"dict".as_slice(), Value::Dict(inner_map));
            let input = Value::Dict(map);

            let mut encoded = Vec::new();
            encode(&input, &mut encoded);

            assert_eq!(encoded, b"d4:dictd3:key5:valueee");
        }

        #[test]
        fn bytes() {
            let input = Value::Bytes(b"banan");

            let mut encoded = Vec::new();
            encode(&input, &mut encoded);

            assert_eq!(encoded, b"5:banan");
        }
    }
}
