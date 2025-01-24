// https://leetcode.com/problems/sudoku-solver/description/
// https://en.wikipedia.org/wiki/Sudoku_solving_algorithms <- Using backtracking algorithm

//Row, Column
pub type Sudoku = [[u8; 9]; 9];

fn validate_point(sudoku: &Sudoku, point: (usize, usize)) -> bool {
    validate_row(sudoku, point.0) && validate_column(sudoku, point.1)
}

fn validate_row(sudoku: &Sudoku, row: usize) -> bool {
    let mut copy = [0_u8; 9];

    for (i, &ele) in sudoku[row].iter().enumerate() {
        if ele == 0_u8 {
            continue;
        }
        if copy.contains(&ele) {
            return false;
        }

        copy[i] = ele;
    }

    true
}

fn validate_column(sudoku: &Sudoku, column: usize) -> bool {
    let mut copy = [0_u8; 9];

    for (i, row) in sudoku.iter().enumerate() {
        let num = row[column];
        if num == 0_u8 {
            continue;
        }
        if copy.contains(&num) {
            return false;
        }
        copy[i] = num;
    }

    return true;
}

fn validate_box(sudoku: &Sudoku) -> bool {
    todo!()
}

fn parse(input: &Vec<Vec<char>>) -> Sudoku {
    let mut sudoku: Sudoku = [[0; 9]; 9];

    for (i, ele) in input.iter().enumerate() {
        for (j, ele1) in ele.iter().enumerate() {
            if ele1.eq(&'.') {
                sudoku[i][j] = 0;
                continue;
            }

            sudoku[i][j] = ele1.to_digit(10).unwrap() as u8;
        }
    }

    sudoku
}

fn parse_back(sudoku: &Sudoku) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 9]; 9];

    for (i, row) in sudoku.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if num != 0 {
                board[i][j] = char::from_digit(num as u32, 10).unwrap();
            }
        }
    }

    board
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut sudoku = parse(board);
    for i in 0..sudoku.len() {
        for j in 0..sudoku[i].len() {
            let num = sudoku[i][j];
            if num != 0 {
                continue;
            }

            sudoku[i][j] += 1;
            while !validate_point(&sudoku, (i, j)) {
                if sudoku[i][j] < 9 {
                    sudoku[i][j] += 1;
                }
                
                
            }
        }
    }

    *board = parse_back(&sudoku);
}
