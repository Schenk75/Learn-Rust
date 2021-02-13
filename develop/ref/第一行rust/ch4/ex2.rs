use std::io::prelude::*;

const BYTES_PRE_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
    println!("hello")
}"#;

fn main() -> std::io::Result<()> {
    let mut butter: Vec<u8> = vec![];
    INPUT.read_to_end(&mut butter);
    let mut position = 0;
    for line in butter.chunks(BYTES_PRE_LINE) {
        print!("[0x{:08x}]", position);
        for byte in line {
            println!("{:02x}", byte);
        }
        println!();
        position += BYTES_PRE_LINE;
    }
    Ok(())
}