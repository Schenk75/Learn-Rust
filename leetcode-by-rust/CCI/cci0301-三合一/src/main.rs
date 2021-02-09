struct TripleInOne {
    d: Vec<i32>,
    i: [usize; 3]
}

impl TripleInOne {
    fn new(stackSize: i32) -> Self {
        Self {
            d: vec![0; 3 * stackSize as usize],
            i: [0, 1, 2]
        }
    }
    
    fn push(&mut self, stack_num: i32, value: i32) {
        let n = stack_num as usize;
        if self.i[n] < self.d.len() {
            self.d[self.i[n]] = value;
            self.i[n] += 3;
        }
    }
    
    fn pop(&mut self, stack_num: i32) -> i32 {
        match stack_num as usize {
            n if self.i[n] >= 3 => {
                self.i[n] -= 3;
                self.d[self.i[n]]
            },
            _ => -1
        }
    }
    
    fn peek(&self, stack_num: i32) -> i32 {
        if self.i[stack_num as usize] >= 3 {self.d[self.i[stack_num as usize] - 3]} else {-1}
    }
    
    fn is_empty(&self, stack_num: i32) -> bool {
        self.i[stack_num as usize] < 3
    }
}

fn main() {
    println!("Hello, world!");
}
