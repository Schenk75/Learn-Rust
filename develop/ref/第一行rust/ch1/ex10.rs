struct Stepper {
    cur: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.max {
            return None
        }
        let res = self.cur;
        self.cur += self.step;
        Some(res)
    }
}

fn main() {
    let mut ex =  Stepper{
        cur: 2,
        step: 3,
        max: 15,
    };
    for item in ex {
        println!("{}", item);
    }
}
