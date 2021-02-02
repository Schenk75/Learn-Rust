use std::cmp::max;

struct Solution;

// // 超出时间限制
// impl Solution {
//     pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
//         let mut max_result = 0;
//         Self::dfs(&grid, 0, 0, 0, &mut max_result);

//         max_result
//     }

//     fn dfs(grid: &Vec<Vec<i32>>, mut i: usize, mut j: usize, mut count: i32, max_result: &mut i32) {
//         count += grid[i][j];
//         if i == grid.len() - 1 && j == grid[0].len() - 1 {
//             if count > *max_result {*max_result = count;}
//             return;
//         }
//         if i == grid.len() - 1 {
//             j += 1;
//             while j < grid[0].len() {
//                 count += grid[i][j];
//                 j += 1;
//             }
//             if count > *max_result {*max_result = count;}
//             return;
//         }
//         if j == grid[0].len() - 1 {
//             i += 1;
//             while i < grid.len() {
//                 count += grid[i][j];
//                 i += 1;
//             }
//             if count > *max_result {*max_result = count;}
//             return;
//         }

//         Self::dfs(grid, i+1, j, count, max_result);
//         Self::dfs(grid, i, j+1, count, max_result);
//     }
// }

// 动态规划
impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        for j in 1..grid[0].len() {
            grid[0][j] = grid[0][j] + grid[0][j-1];
        }
        for i in 1..grid.len() {
            grid[i][0] = grid[i][0] + grid[i-1][0];
        }

        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                grid[i][j] = grid[i][j] + max(grid[i-1][j], grid[i][j-1]);
            }
        }

        *grid.last().unwrap().last().unwrap()
    }
}

fn main() {
    let grid = vec![
        vec![1,3,1],
        vec![1,5,1],
        vec![4,2,1]
    ];
    println!("{}", Solution::max_value(grid));
}
