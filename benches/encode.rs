use std::collections::BTreeMap;

use criterion::{Criterion, criterion_group, criterion_main};
use superbencode::{Value, encode as ec};

fn encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    let val = Value::Bytes(&[42; 10_000]);
    group.bench_function("bytes", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            ec(&val, &mut buf);
        });
    });

    let val = Value::Int(409_218_498_421);
    group.bench_function("int", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            ec(&val, &mut buf);
        });
    });

    let inner_inner = vec![Value::List(Vec::new()); 100];
    let inner = vec![Value::List(inner_inner.clone()); 100];
    let val = Value::List(inner);
    group.bench_function("list", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            ec(&val, &mut buf);
        });
    });

    let inner_inner: [_; 100] =
        std::array::from_fn(|_| (b"".as_slice(), Value::Dict(BTreeMap::new())));
    let inner_inner = BTreeMap::from(inner_inner);
    let inner: [_; 100] =
        std::array::from_fn(|_| (b"".as_slice(), Value::Dict(inner_inner.clone())));
    let inner = BTreeMap::from(inner);
    let val = Value::Dict(inner);
    group.bench_function("dict", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            ec(&val, &mut buf);
        });
    });

    group.finish();
}

const TIME: std::time::Duration = std::time::Duration::from_secs(20);
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(500).measurement_time(TIME);
    targets = encode
}
criterion_main!(benches);
