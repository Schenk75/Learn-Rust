pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if dfs(&board, &chars, i as i32, j as i32, 0) {return true;}
        }
    }

    false
}

fn dfs(board: &Vec<Vec<char>>, chs: &Vec<char>, i: i32, j: i32, k: usize) -> bool {
    let mut board = board.clone();
    if i < 0 || i >= board.len() as i32 || j < 0 || j >= board[0].len() as i32 || board[i as usize][j as usize] != chs[k] {return false;}
    if k == chs.len() - 1 {return true;}
    board[i as usize][j as usize] = '\0';
    dfs(&board, chs, i+1, j, k+1) || dfs(&board, chs, i, j+1, k+1) || dfs(&board, chs, i-1, j, k+1) || dfs(&board, chs, i, j-1, k+1)
}

fn main() {
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']];
    let word = "ABCCED".to_string();
    println!("{}", exist(board, word));
}
