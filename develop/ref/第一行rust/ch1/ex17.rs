use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();// 注意这种写法
        println!("{} ({} bytes long)", line, line.len());
    }
}