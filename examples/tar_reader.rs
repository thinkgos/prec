use std::fs::File;
use std::io::prelude::*;
use tar::Archive;

fn main() {
    let file = File::open("testdata/foo.tar").unwrap();
    let mut a = Archive::new(file);

    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        // Inspect metadata about the file
        println!("file path: {:?}", file.header().path().unwrap());
        println!("file size: {}", file.header().size().unwrap());

        // files implement the Read trait
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        println!("file content: {}\n", s);
    }
}
