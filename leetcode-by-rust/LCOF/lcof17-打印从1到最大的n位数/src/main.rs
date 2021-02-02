struct Solution;

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let x = 10i32.saturating_pow(n as u32);
        for i in 1..x {
            result.push(i);
        }

        result
    }

    pub fn bn_print_numbers(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut num = Vec::new();

        Self::dfs(0, n, &mut result, &mut num);

        result
    }

    fn dfs(x: i32, n: i32, result: &mut Vec<i32>, num: &mut Vec<char>) {
        if x == n {
            while num.len() != 0 && num[0] == '0' {
                num.remove(0);
            }
            if num.len() != 0 {
                let tmp: String = num.iter().collect();
                result.push(tmp.parse().unwrap());
            }

            return;
        }

        for i in vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            num.push(i);
            Self::dfs(x+1, n, result, num);

            if num.len() != 0 {
                num.remove(num.len()-1);
            }
        }
    }
}

fn main() {
    let n = 2;
    println!("{:?}", Solution::print_numbers(n));
    println!("{:?}", Solution::bn_print_numbers(n));
}
