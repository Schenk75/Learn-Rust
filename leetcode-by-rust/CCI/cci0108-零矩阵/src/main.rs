use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {return;}
        let mut set_col = HashSet::new();
        let mut set_row = HashSet::new();
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    set_col.insert(j);
                    set_row.insert(i);
                    break;
                }
            }
        }

        for i in set_row {
            for k in 0..n {
                matrix[i][k] = 0;
            }
        }
        for j in set_col {
            for k in 0..m {
                matrix[k][j] = 0;
            }
        }
    }
}

fn main() {
    let mut matrix = vec![
        vec![1,1,1],
        vec![1,0,1],
        vec![1,1,1]
      ];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}
