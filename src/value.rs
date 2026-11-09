use std::{collections::BTreeMap, ops::Index};

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'buf> {
    Bytes(&'buf [u8]),

    // The specification does not include a limit on the size of an integer.
    // Arbitrary size integers could be used, but there are not enough use cases to be worth the cost.
    Int(i64),
    List(Vec<Value<'buf>>),
    Dict(BTreeMap<&'buf [u8], Value<'buf>>)
}

impl<'buf> Index<&[u8]> for Value<'buf> {
    type Output = Value<'buf>;

    fn index(&self, index: &[u8]) -> &Self::Output {
        if let Value::Dict(map) = self {
            &map[index]
        } else {
            panic!("indexing with dictionary key into non-dictionary value")
        }
    }
}

impl<'buf> Index<usize> for Value<'buf> {
    type Output = Value<'buf>;

    fn index(&self, index: usize) -> &Self::Output {
        if let Value::List(list) = self {
            &list[index]
        } else {
            panic!("indexing with list index into non-list value")
        }
    }
}

impl<'buf, const N: usize> Index<[u8; N]> for Value<'buf> {
    type Output = Value<'buf>;

    fn index(&self, index: [u8; N]) -> &Self::Output {
        &self[index.as_slice()]
    }
}

impl<'buf, const N: usize> Index<&[u8; N]> for Value<'buf> {
    type Output = Value<'buf>;

    fn index(&self, index: &[u8; N]) -> &Self::Output {
        &self[*index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod index {
        use super::*;

        #[test]
        fn bytes() {
            let mut map = BTreeMap::new();
            map.insert(b"hello".as_slice(), Value::Int(42));
            let i1 = Value::Dict(map);

            let o1 = &i1[b"hello"];
            let o2 = &i1[*b"hello"];
            let o3 = &i1[b"hello".as_slice()];

            let expected = Value::Int(42);
            assert_eq!(*o1, expected);
            assert_eq!(*o2, expected);
            assert_eq!(*o3, expected);
        }

        #[test]
        fn usize() {
            let list = Value::List(vec![Value::Int(42)]);

            let o1 = &list[0];

            assert_eq!(*o1, Value::Int(42));
        }

        #[test]
        #[should_panic = "no entry found for key"]
        fn bad_bytes_1() {
            let mut map = BTreeMap::new();
            map.insert(b"hello".as_slice(), Value::Int(42));
            let i1 = Value::Dict(map);

            let _ = &i1[b"notpresent"];
        }

        #[test]
        #[should_panic = "indexing with dictionary key into non-dictionary value"]
        fn bad_bytes_2() {
            let i1 = Value::Int(42);

            let _ = &i1[b"this_is_not_a_dict"];
        }

        #[test]
        #[should_panic = "index out of bounds"]
        fn bad_usize_1() {
            let list = Value::List(vec![Value::Int(42)]);

            let _ = &list[1];
        }

        #[test]
        #[should_panic = "indexing with list index into non-list value"]
        fn bad_usize_2() {
            let list = Value::Int(42);

            let _ = &list[0];
        }
    }
}
