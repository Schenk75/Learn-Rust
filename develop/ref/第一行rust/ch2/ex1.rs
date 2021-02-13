#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

use std::fmt::Debug;
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () { // 可以不写
    *text = String::from("");
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
//read(f1, vec![]); ❽
    close(&mut f1);
}