mod common;

use std::hint::black_box;

use common::{bendy_encode, bendy_owned, bendy_zc, inputs, sanity_check};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_decode");
    for (name, data) in inputs() {
        sanity_check(&data);
        group.throughput(Throughput::Bytes(data.len() as u64));
        let d = data.as_slice();

        group.bench_with_input(BenchmarkId::new("superbencode", &name), d, |b, d| {
            b.iter_with_large_drop(|| superbencode::decode(black_box(d)).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("serde_bencode", &name), d, |b, d| {
            b.iter_with_large_drop(|| {
                serde_bencode::from_bytes::<serde_bencode::value::Value>(black_box(d)).unwrap()
            });
        });
        group.bench_with_input(BenchmarkId::new("bendy", &name), d, |b, d| {
            b.iter_with_large_drop(|| bendy_owned(black_box(d)).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("bendy_zc", &name), d, |b, d| {
            b.iter_with_large_drop(|| bendy_zc(black_box(d)).unwrap());
        });
    }
    group.finish();
}

fn encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_encode");
    for (name, data) in inputs() {
        group.throughput(Throughput::Bytes(data.len() as u64));

        let sb = superbencode::decode(&data).unwrap();
        group.bench_function(BenchmarkId::new("superbencode", &name), |b| {
            b.iter(|| {
                let mut buf = Vec::new();
                superbencode::encode(black_box(&sb), &mut buf);
                buf
            });
        });

        let sv: serde_bencode::value::Value = serde_bencode::from_bytes(&data).unwrap();
        group.bench_function(BenchmarkId::new("serde_bencode", &name), |b| {
            b.iter(|| serde_bencode::to_bytes(black_box(&sv)).unwrap());
        });

        let bv = bendy_zc(&data).unwrap();
        group.bench_function(BenchmarkId::new("bendy", &name), |b| {
            b.iter(|| bendy_encode(black_box(&bv)));
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(5));
    targets = decode, encode
}
criterion_main!(benches);
