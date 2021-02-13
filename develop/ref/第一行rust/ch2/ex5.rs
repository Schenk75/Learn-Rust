use rand::Rng;
use std::io::Read;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug)]
struct FileStruct {
    name: String,
    data: Vec<u8>
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = std::fs::File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

impl FileStruct {
    fn new(name: &str) -> FileStruct {
        FileStruct {name: String::from(name), data: Vec::new()}
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> FileStruct {
        let mut f = FileStruct::new(name);
        f.data = data.clone();
        f
    }
    fn read(self: &FileStruct, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: FileStruct) -> Result<FileStruct, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: FileStruct) -> Result<FileStruct, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = FileStruct::new_with_data("test.txt", &f4_data);
    let mut buffer: Vec<u8> = vec![];
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();
    let text = String::from_utf8_lossy(&buffer); //将字节的一部分转换为字符串，包括无效字符。
    println!("{:?}", f4);
    println!("{} is {} bytes long", f4.name, f4_length);
    println!("{}", text);
    let text_context = get_file_as_byte_vec(&"readme.md".to_string());
    println!("{}", String::from_utf8_lossy(&text_context));
}