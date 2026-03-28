use superbencode::{decode, encode};

fn main() {
    let test = include_bytes!("../../test/bbb_sunflower.torrent");

    let decoded = decode(test).unwrap();
    let mut encoded = Vec::new();
    encode(&decoded, &mut encoded);

    std::fs::write("/dev/null", &encoded).unwrap(); // prevent optimizations
}
