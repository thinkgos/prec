use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::prelude::*;
fn main() {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"foo").unwrap();
    e.write_all(b"bar").unwrap();
    let compressed_bytes = e.finish().unwrap();
    fs::write("testdata/foo.gz", compressed_bytes).unwrap();
}
