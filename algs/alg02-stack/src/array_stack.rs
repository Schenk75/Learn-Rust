// 用数组实现一个顺序栈
#[derive(Debug)]
pub struct ArrayStack {
    data: Vec<i32>,
    top: i32,
}

impl ArrayStack {
    pub fn new() -> Self {
        ArrayStack {data: Vec::with_capacity(32), top: -1}
    }

    fn is_empty(&self) -> bool {
        if self.top < 0 {true} else {false}
    }

    pub fn push(&mut self, x: i32) {
        self.top += 1;
        if self.top == self.data.capacity() as i32 {
            let temp = self.data.clone();
            self.data = Vec::with_capacity(self.data.capacity() * 2);
            for &val in temp.iter() {
                self.data.push(val);
            }
        }
        self.data.push(x);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {return None;}
        self.top -= 1;
        self.data.pop()
    }

    pub fn top(&self) -> Option<i32> {
        if self.is_empty() {return None;}
        Some(self.data[self.top as usize])
    }
}
