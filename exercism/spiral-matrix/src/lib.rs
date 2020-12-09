enum Command {
    Right,
    Left,
    Up,
    Down,
}

impl Command {
    fn next(&self) -> Self {
        match self {
            Command::Right => Command::Down,
            Command::Down => Command::Left,
            Command::Left => Command::Up,
            Command::Up => Command::Right,
        }
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut mat: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let mut num = 1;
    let mut row = 0;
    let mut column = 0;
    let mut command = Command::Right;
    let mut distance = size as usize;

    while distance > 0 {
        match command {
            Command::Right => {
                for _ in 0..distance-1 {
                    mat[row][column] = num;
                    num += 1;
                    column += 1;
                }
                mat[row][column] = num;
                num += 1;
                row += 1;
                distance -= 1;
            },
            Command::Down => {
                for _ in 0..distance-1 {
                    mat[row][column] = num;
                    num += 1;
                    row += 1;
                }
                mat[row][column] = num;
                num += 1;
                column = column.saturating_sub(1);
            },
            Command::Left => {
                for _ in 0..distance-1 {
                    mat[row][column] = num;
                    num += 1;
                    column = column.saturating_sub(1);
                }
                mat[row][column] = num;
                num += 1;
                row = row.saturating_sub(1);
                distance -= 1;
            },
            Command::Up => {
                for _ in 0..distance-1 {
                    mat[row][column] = num;
                    num += 1;
                    row = row.saturating_sub(1);
                }
                mat[row][column] = num;
                num += 1;
                column += 1;
            },
        }
        command = command.next();
    }

    mat
}
