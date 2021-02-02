struct Solution;

impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        let num_str: Vec<i32> = num.to_string().chars().into_iter().map(|x| x.to_digit(10).unwrap() as i32).collect();
        // println!("{:?}", num_str);
        let mut count = 0;
        Self::dfs(&num_str, 0, &mut count);
        count
    }

    fn dfs(num_str: &Vec<i32>, x: i32, count: &mut i32) {
        if x == num_str.len() as i32 {
            *count += 1;
            return;
        }

        Self::dfs(num_str, x+1, count);
        if num_str[x as usize] != 0 && x+1 < num_str.len() as i32 && num_str[x as usize]*10 + num_str[(x+1) as usize] <= 25 {
            Self::dfs(num_str, x+2, count);
        }
        
    }
}

fn main() {
    let num = 506;
    println!("{}", Solution::translate_num(num));
}
