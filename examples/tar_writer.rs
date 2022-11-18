use std::fs::File;
use std::io::prelude::*;
use tar::Builder;

fn main() {
    let file = File::create("testdata/foo.tar").unwrap();
    let mut a = Builder::new(file);

    a.append_path("testdata/file1.txt").unwrap();
    a.append_file("file2.txt", &mut File::open("testdata/file2.txt").unwrap())
        .unwrap();
}
