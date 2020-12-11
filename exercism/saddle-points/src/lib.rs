use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if input.is_empty() { return result; }
    if input[0].is_empty() { return result; }

    let line_num = input.len();
    let column_num = input[0].len();
    let mut temp_result: Vec<(usize, usize)> = Vec::new();
    
    for i in 0..line_num {
        let mut max = input[i][0];
        let mut temp = vec![(i, 0)];
        for j in 1..column_num {
            if input[i][j] > max {
                temp.clear();
                temp.push((i, j));
                max = input[i][j];
            } else if input[i][j] == max {
                temp.push((i, j));
            }
        }
        temp_result.extend(temp);
    }

    for j in 0..column_num {
        let mut min = input[0][j];
        let mut temp = vec![(0, j)];
        for i in 1..line_num {
            if input[i][j] < min {
                temp.clear();
                temp.push((i, j));
                min = input[i][j];
            } else if input[i][j] == min {
                temp.push((i, j));
            }
        }
        temp_result.extend(temp);
    }

    let mut map = HashMap::new();
    for item in temp_result.iter() {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }
    
    for (key, val) in map {
        if val == 2 {
            result.push(*key);
        }
    }

    result
}
