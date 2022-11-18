use flate2::read::GzDecoder;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let f = File::open("testdata/foo.gz").unwrap();
    let mut d = GzDecoder::new(f);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
