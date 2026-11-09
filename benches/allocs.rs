//! Counts heap allocations made by each crate to decode / encode the same input.
//!
//! Run with `cargo bench --bench allocs`. This is a deterministic count, not a
//! timing benchmark, so each operation is run exactly once per input.

mod common;

use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed}
};

use common::{bendy_encode, bendy_owned, bendy_zc, inputs, sanity_check};

struct Counting;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

fn grow_live(n: usize) {
    let live = LIVE.fetch_add(n, Relaxed) + n;
    PEAK.fetch_max(live, Relaxed);
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        grow_live(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        grow_live(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Relaxed);
        if new_size > layout.size() {
            BYTES.fetch_add(new_size - layout.size(), Relaxed);
            grow_live(new_size - layout.size());
        } else {
            LIVE.fetch_sub(layout.size() - new_size, Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

struct Stats {
    allocs:   usize,
    reallocs: usize,
    bytes:    usize,
    peak:     usize
}

fn measure<T>(f: impl FnOnce() -> T) -> Stats {
    let base = LIVE.load(Relaxed);
    ALLOCS.store(0, Relaxed);
    REALLOCS.store(0, Relaxed);
    BYTES.store(0, Relaxed);
    PEAK.store(base, Relaxed);

    let out = black_box(f());
    let stats = Stats {
        allocs:   ALLOCS.load(Relaxed),
        reallocs: REALLOCS.load(Relaxed),
        bytes:    BYTES.load(Relaxed),
        peak:     PEAK.load(Relaxed) - base
    };
    drop(out);
    stats
}

fn kib(n: usize) -> String {
    #[allow(clippy::cast_precision_loss)]
    let k = n as f64 / 1024.0;
    format!("{k:.1} KiB")
}

fn row(op: &str, input: &str, krate: &str, s: &Stats) {
    println!(
        "{op:<7}{input:<10}{krate:<15}{:>9}{:>10}{:>14}{:>14}",
        s.allocs,
        s.reallocs,
        kib(s.bytes),
        kib(s.peak)
    );
}

fn main() {
    let inputs = inputs();
    for (_, data) in &inputs {
        sanity_check(data);
    }

    println!(
        "{:<7}{:<10}{:<15}{:>9}{:>10}{:>14}{:>14}",
        "op", "input", "crate", "allocs", "reallocs", "bytes alloc'd", "peak live"
    );
    for (name, data) in &inputs {
        let d = data.as_slice();
        row(
            "decode",
            name,
            "superbencode",
            &measure(|| superbencode::decode(d).unwrap())
        );
        row(
            "decode",
            name,
            "serde_bencode",
            &measure(|| serde_bencode::from_bytes::<serde_bencode::value::Value>(d).unwrap())
        );
        row(
            "decode",
            name,
            "bendy",
            &measure(|| bendy_owned(d).unwrap())
        );
        row(
            "decode",
            name,
            "bendy_zc",
            &measure(|| bendy_zc(d).unwrap())
        );
    }
    println!();
    for (name, data) in &inputs {
        let sb = superbencode::decode(data).unwrap();
        row(
            "encode",
            name,
            "superbencode",
            &measure(|| {
                let mut buf = Vec::new();
                superbencode::encode(&sb, &mut buf);
                buf
            })
        );
        let sv: serde_bencode::value::Value = serde_bencode::from_bytes(data).unwrap();
        row(
            "encode",
            name,
            "serde_bencode",
            &measure(|| serde_bencode::to_bytes(&sv).unwrap())
        );
        let bv = bendy_zc(data).unwrap();
        row("encode", name, "bendy", &measure(|| bendy_encode(&bv)));
    }
}
