struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        if matrix.is_empty() {return result;}

        let (mut right, mut left, mut down, mut up) = ((matrix[0].len()-1) as i32, 0, (matrix.len()-1) as i32, 0);

        loop {
            for j in left..=right {
                result.push(matrix[up as usize][j as usize]);
            }
            up += 1;
            if up > down {break;}

            for i in up..=down {
                result.push(matrix[i as usize][right as usize]);
            }
            right -= 1;
            if left > right {break;}

            let mut j = right;
            while j >= left {
                result.push(matrix[down as usize][j as usize]);
                j -= 1;
            }
            down -= 1;
            if up > down {break;}

            let mut i = down;
            while i >= up {
                result.push(matrix[i as usize][left as usize]);
                i -= 1;
            }
            left += 1;
            if left > right {break;}
        }

        result
    }
}

fn main() {
    let matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9]];
    // println!("{:?}", Solution::spiral_order(matrix));
    assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,6,9,8,7,4,5]);

    let matrix = vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12]];
    println!("{:?}", Solution::spiral_order(matrix));
}
