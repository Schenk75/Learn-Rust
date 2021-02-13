use std::io::{Read, Bytes};
use std::fmt::Debug;

#[derive(Debug)]
struct File {
    name: String,
    data: Bytes<std::fs::File>,
}

impl File {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let file_name = String::from("readme.md");
    let file = std::fs::File::open(&file_name).unwrap();
    let f1 = File {
        name: file_name,
        data: file.bytes(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.count();
    println!("file name: {}, file length : {}", f1_name, f1_length)
}