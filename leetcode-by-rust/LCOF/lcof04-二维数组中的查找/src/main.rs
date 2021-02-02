pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() {return false;}
    
    let mut idx_row = 0;
    // 先横着找得到横边界
    for i in 0..matrix[0].len() {
        if target > matrix[0][i] {
            idx_row = i;
        } else if target < matrix[0][i] {
            break;
        } else {
            return true;
        }
    }

    // 竖着找
    for i in 0..matrix.len() {
        if matrix[0].is_empty() {return false;}
        if target > matrix[i][0] {
            for j in 1..=idx_row {
                if target < matrix[i][j]{
                    break;
                } else if target == matrix[i][j] {
                    return true;
                }
            }
        } else if target < matrix[i][0] {
            break;
        } else {
            return true;
        }
    }
    false
}

// 从二维数组的右上角开始查找。如果当前元素等于目标值，则返回 true。如果当前元素大于目标值，则移到左边一列。如果当前元素小于目标值，则移到下边一行。
pub fn find_number_in2_d_array_improve(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() {return false;}

    let mut row = 0;
    let mut col = (matrix[0].len()-1) as i32;

    while (row as usize) < matrix.len() && col >= 0 {
        if target > matrix[row as usize][col as usize] {
            row += 1;
        } else if target < matrix[row as usize][col as usize] {
            col -= 1;
        } else {
            return true;
        }
    }

    false
}

fn main() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30]
    ];
    // let matrix = vec![
    //     vec![1, 2, 3, 4, 5],
    //     vec![6,7,8,9,10],
    //     vec![11,12,13,14,15],
    //     vec![16,17,18,19,20],
    //     vec![21,22,23,24,25]
    // ];
    // let matrix = vec![
    //     vec![1,1]
    // ];
    // let matrix = vec![
    //     vec![2,5],
    //     vec![2,8],
    //     vec![7,9],
    //     vec![7,11],
    //     vec![9,11]
    // ];
    // let matrix = vec![
    //     vec![1,3,5,7,9],
    //     vec![2,4,6,8,10],
    //     vec![11,13,15,17,19],
    //     vec![12,14,16,18,20],
    //     vec![21,22,23,24,25]
    // ];
    // println!("{}", find_number_in2_d_array(matrix.clone(), 13));
    // println!("{}", find_number_in2_d_array(matrix.clone(), 20));
    println!("{}", find_number_in2_d_array_improve(matrix.clone(), 5));
    println!("{}", find_number_in2_d_array_improve(matrix.clone(), 20));
}
