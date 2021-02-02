struct Solution;

impl Solution {
    pub fn dices_probability(n: i32) -> Vec<f64> {
        let len = 6*n as usize;
        let mut probs = vec![0; len];
        let mut result: Vec<f64> = vec![];

        for i in 0..6 {
            probs[i] = 1;
        }

        // n个骰子和为m的情况数 = n-1个骰子和为m-1, m-2, m-3, m-4, m-5, m-6的情况数之和
        for _i in 2..=n as usize {
            let tmp = probs.clone();
            for j in 0..len {
                probs[j] = 0;
                for k in 1..=6 {
                    if j >= k {
                        probs[j] += tmp[j-k];
                    } else {break;}
                }
            }
        }
        println!("{:?}", probs);

        for prob in probs {
            if prob != 0 {result.push(prob as f64 / 6f64.powi(n));}
        }

        result
    }
}

fn main() {
    let n = 2;
    println!("{:?}", Solution::dices_probability(n));
}
