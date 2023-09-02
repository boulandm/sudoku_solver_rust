pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    solve(board, 0, 0);
}

fn solve(board: &mut Vec<Vec<char>>, r: usize, c: usize) -> bool {
    return if r == 9 {
        true
    } else if c == 9 {
        solve(board, r + 1, 0)
    } else if board[r][c] != '.' {
        solve(board, r, c + 1)
    } else {
        for k in 1..10 {
            let x = match char::from_digit(k, 10) {
                Some(y) => y,
                None => '.'
            };

            if is_valid(board, r, c, &x) {
                board[r][c] = x;
                if solve(board, r, c + 1) {
                    return true;
                }
                board[r][c] = '.';
            }
        }

        false
    };
}

pub fn is_valid(board: &mut Vec<Vec<char>>, r: usize, c: usize, k: &char) -> bool {
    if board[r].contains(&k) {
        return false;
    }

    for row in 0..9 {
        if board[row][c] == *k {
            return false;
        }
    }

    for row in ((r / 3) * 3)..(((r / 3) + 1) * 3) {
        for column in ((c / 3) * 3)..(((c / 3) +1 ) * 3) {
            if &board[row][column] == k {
                return false;
            }
        }
    }

    true
}

pub fn print_sudoku(board: &mut Vec<Vec<char>>) {
    println!();
    for row in board {
        for cell in row {
            print!("{} ", cell)
        }
        println!();
    }
    println!();
}