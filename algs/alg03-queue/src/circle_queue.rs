use std::fmt;

#[derive(Debug)]
pub struct CircleQueue {
    queue: Vec<i32>,
    head: usize,
    tail: usize,
    n: usize,
}
     
impl CircleQueue {
    pub fn new(n: usize) -> Self {
        CircleQueue {
            queue: vec![-1; n+1],
            head: 0,
            tail: 0,
            n: n+1,
        }
    }

    pub fn enqueue(&mut self, num: i32) -> bool {
        if (self.tail + 1) % self.n == self.head {return false;}
        self.queue[self.tail] = num;
        self.tail = (self.tail + 1) % self.n;
        true
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.head == self.tail {return None;}

        let shift = self.queue[self.head];
        self.head = (self.head + 1) % self.n;
        Some(shift)
    }
}

impl fmt::Display for CircleQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut result = String::new();
        if self.head < self.tail {
            for i in self.head..self.tail {
                if result.is_empty() {
                    result = format!("{}", self.queue[i]); 
                } else {
                    result = format!("{}->{}", result, self.queue[i]);
                }
            }
        } else if self.head > self.tail {
            for i in self.head..self.queue.len() {
                if result.is_empty() {
                    result = format!("{}", self.queue[i]); 
                } else {
                    result = format!("{}->{}", result, self.queue[i]);
                }
            }
            for i in 0..self.tail {
                result = format!("{}->{}", result, self.queue[i]);
            }
        }
        
        write!(f, "{} head:{} tail:{}", result, self.head, self.tail)
    }
}