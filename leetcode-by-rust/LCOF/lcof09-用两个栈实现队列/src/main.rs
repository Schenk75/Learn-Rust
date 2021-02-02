struct CQueue {
    p: Vec<i32>,
    q: Vec<i32>
}

impl CQueue {
    fn new() -> Self {
        CQueue {p: vec![], q: vec![]}
    }
    
    fn append_tail(&mut self, value: i32) {
        self.p.push(value);
    }
    
    fn delete_head(&mut self) -> i32 {
        match self.q.pop() {
            Some(val) => val,
            None => {
                while !self.p.is_empty() {
                    self.q.push(self.p.pop().unwrap());
                }
                match self.q.pop() {
                    Some(val) => val,
                    None => -1
                }
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
}
