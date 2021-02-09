struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut tmp;
        let len = matrix.len();

        for i in 0..len/2 {
            for j in 0..(len+1)/2 {
                tmp = matrix[i][j];
                matrix[i][j] = matrix[len-j-1][i];
                matrix[len-j-1][i] = matrix[len-i-1][len-j-1];
                matrix[len-i-1][len-j-1] = matrix[j][len-i-1];
                matrix[j][len-i-1] = tmp;   
            }
        }
    }
}

fn main() {
    let mut matrix = vec![
        vec![ 5, 1, 9,11],
        vec![ 2, 4, 8,10],
        vec![13, 3, 6, 7],
        vec![15,14,12,16]
      ];
    Solution::rotate(&mut matrix);
    println!("{:?}", matrix);
}
