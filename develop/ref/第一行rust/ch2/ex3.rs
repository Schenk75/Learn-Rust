use rand::random;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}
static mut ERROR: usize = 0;

impl File {
    fn new(name: &str) -> File {
        File{name: String::from(name), data: Vec::new()}
    }
}

#[allow(unused_variables)]
fn read(f: &File, save: &mut Vec<u8>) -> usize {
    if !(random() && random() && random() && random()) {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[allow(unused_mut)]
fn main() {
    let mut f = File::new("test.txt");
    let mut buffer = vec![];
    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}