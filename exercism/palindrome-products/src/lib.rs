#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    val: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            val: a * b,
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut pmin = Palindrome::new(max-1, max);
    let mut pmax = Palindrome::new(0, 0);

    for i in min..=max {
        for j in i..=max {
            let p = i * j;
            if pmin.val == p {
                pmin.insert(i, j);
            } else if pmin.val > p && is_palindrome(p) {
                pmin = Palindrome::new(i, j);
            }

            if pmax.val == p {
                pmax.insert(i, j);
            } else if pmax.val < p && is_palindrome(p) {
                pmax = Palindrome::new(i, j);
            }
        }
    }

    if pmax.val > 0 {
        return Some((pmin, pmax));
    } else {
        return None;
    }
}

fn is_palindrome(num: u64) -> bool {
    let s: String = num.to_string();
    let rev_s: String = s.chars().rev().collect();
    
    s == rev_s
}