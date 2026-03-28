use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use superbencode::{Value, decode as dc, encode as ec};

const VALUE: &[u8] = include_bytes!("../test/bbb_sunflower.torrent");

fn roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_roundtrip");
    group.throughput(Throughput::Bytes(VALUE.len() as u64));

    group.bench_function("decode_then_encode", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            let val = dc(VALUE).unwrap();
            ec(&val, &mut buf);
        });
    });

    group.finish();
}

fn scaling(c: &mut Criterion) {
    let val = dc(VALUE).unwrap();

    // build inputs of varying sizes by repeating the torrent value in a list
    let mut group = c.benchmark_group("file_decode_scaling");
    for count in [4, 16, 256] {
        let list = Value::List(vec![val.clone(); count]);
        let mut encoded = Vec::new();
        ec(&list, &mut encoded);
        group.throughput(Throughput::Bytes(encoded.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &encoded, |b, data| {
            b.iter_with_large_drop(|| dc(data.as_slice()).unwrap());
        });
    }
    group.finish();

    let mut group = c.benchmark_group("file_encode_scaling");
    for count in [4, 16, 256] {
        let list = Value::List(vec![val.clone(); count]);
        let mut size_buf = Vec::new();
        ec(&list, &mut size_buf);
        let size = size_buf.len() as u64;
        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(BenchmarkId::from_parameter(count), &list, |b, data| {
            b.iter(|| {
                let mut buf = Vec::new();
                ec(data, &mut buf);
            });
        });
    }
    group.finish();
}

const TIME: std::time::Duration = std::time::Duration::from_secs(20);
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(500).measurement_time(TIME);
    targets = roundtrip, scaling
}
criterion_main!(benches);
