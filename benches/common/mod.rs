//! Helpers shared by the cross-crate comparison benches.

use std::{borrow::Cow, collections::BTreeMap};

use bendy::{
    decoding::{Decoder, FromBencode, Object},
    encoding::ToBencode
};
use superbencode::Value;

pub const VALUE: &[u8] = include_bytes!("../../test/bbb_sunflower.torrent");

fn bendy_zc_object<'a>(obj: Object<'_, 'a>) -> Option<bendy::value::Value<'a>> {
    use bendy::value::Value as V;
    Some(match obj {
        Object::Bytes(b) => V::Bytes(Cow::Borrowed(b)),
        Object::Integer(i) => V::Integer(i.parse().ok()?),
        Object::List(mut l) => {
            let mut v = Vec::new();
            while let Some(o) = l.next_object().ok()? {
                v.push(bendy_zc_object(o)?);
            }
            V::List(v)
        }
        Object::Dict(mut d) => {
            let mut m = BTreeMap::new();
            while let Some((k, o)) = d.next_pair().ok()? {
                m.insert(Cow::Borrowed(k), bendy_zc_object(o)?);
            }
            V::Dict(m)
        }
    })
}

/// `Value::from_bencode` uses a max depth of 0 and rejects any nesting, so use
/// the decoder directly with its default depth limit.
pub fn bendy_owned(bytes: &[u8]) -> Option<bendy::value::Value<'static>> {
    let mut dec = Decoder::new(bytes);
    bendy::value::Value::decode_bencode_object(dec.next_object().ok()??).ok()
}

/// Same depth problem as `from_bencode`: `to_bencode` caps depth at 0.
pub fn bendy_encode(val: &bendy::value::Value<'_>) -> Vec<u8> {
    let mut enc = bendy::encoding::Encoder::new().with_max_depth(64);
    enc.emit_with(|e| val.encode(e)).unwrap();
    enc.get_output().unwrap()
}

/// Zero-copy tree built on bendy's streaming decoder.
pub fn bendy_zc(bytes: &[u8]) -> Option<bendy::value::Value<'_>> {
    let mut dec = Decoder::new(bytes);
    bendy_zc_object(dec.next_object().ok()??)
}

/// Builds the benchmark inputs: the raw torrent, plus lists of N copies of it
pub fn inputs() -> Vec<(String, Vec<u8>)> {
    let val = superbencode::decode(VALUE).unwrap();
    let mut out = vec![("torrent".to_owned(), VALUE.to_vec())];
    for n in [16, 256] {
        let mut buf = Vec::new();
        superbencode::encode(&Value::List(vec![val.clone(); n]), &mut buf);
        out.push((format!("list_x{n}"), buf));
    }
    out
}

pub fn sanity_check(data: &[u8]) {
    // Every crate must decode and encode the input correctly
    let mut sb = Vec::new();
    superbencode::encode(&superbencode::decode(data).unwrap(), &mut sb);
    assert_eq!(sb, data, "superbencode");

    let sv: serde_bencode::value::Value = serde_bencode::from_bytes(data).unwrap();
    assert_eq!(serde_bencode::to_bytes(&sv).unwrap(), data, "serde_bencode");

    let bv = bendy_owned(data).unwrap();
    assert_eq!(bendy_encode(&bv), data, "bendy");

    assert_eq!(bendy_encode(&bendy_zc(data).unwrap()), data, "bendy_zc");
}
