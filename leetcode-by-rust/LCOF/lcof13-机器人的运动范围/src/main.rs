use std::collections::HashSet;

pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    let mut result: i32 = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    dfs(&mut visited, m, n, k, 0, 0, &mut result);
    result
}

fn add(mut m: i32, mut n: i32) -> i32 {
    let mut result = 0;
    while m != 0 {
        result += m % 10;
        m /= 10;
    }
    while n != 0 {
        result += n % 10;
        n /= 10;
    }
    result
}

fn dfs(visited: &mut HashSet<(i32, i32)>, m: i32, n: i32, k: i32, i: i32, j: i32, result: &mut i32) {
    if i < 0 || i >= m || j < 0 || j >= n {return;}

    if add(i, j) <= k && !visited.contains(&(i, j)) {
        visited.insert((i, j));
        *result += 1;
    } else {
        return;
    }

    for (x, y) in [(1, 0), (0, 1)].iter() {
        dfs(visited, m, n, k, i+x, j+y, result);
    }
}

pub fn bfs_moving_count(m: i32, n: i32, k: i32) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut result = 0;
    let mut queue = vec![(0, 0)];

    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();
        if add(i, j) <= k && !visited.contains(&(i, j)) {
            visited.insert((i, j));
            result += 1;
        } else {
            continue;
        }
        if i + 1 < m {queue.push((i+1, j));}
        if j + 1 < n {queue.push((i, j+1));}
    }

    result
}

fn main() {
    let (m, n, k) = (36, 11, 15);
    println!("{}", moving_count(m, n, k));
    println!("{}", bfs_moving_count(m, n, k));
}
