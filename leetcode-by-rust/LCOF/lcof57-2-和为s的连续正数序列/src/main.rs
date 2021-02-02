struct Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let (mut i, mut j) = (1, 2);
        let mut sum;
        let mut result = vec![];

        while i <= target / 2 {
            sum = (i + j) * (j - i + 1) / 2;
            println!("{}+..+{}={}", i, j, sum);
            if sum == target {
                result.push((i..=j).collect());
                i += 1;
            } else if sum < target {
                j += 1;
            } else {
                i += 1;
            }
        }

        result
    }
}

fn main() {
    let target = 1;
    println!("{:?}", Solution::find_continuous_sequence(target));
}
