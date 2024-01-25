// The board dimensions.
pub const NUM_ROWS: usize = 15;
pub const NUM_COLS: usize = NUM_ROWS;
pub const INUM_ROWS: i32 = NUM_ROWS as i32;
pub const INUM_COLS: i32 = NUM_COLS as i32;

pub const EMPTY_SQUARE: &str = " □";
pub const QUEEN_SQUARE: &str = " ♕";

pub fn dump_board(board: &mut [[&str; NUM_COLS]; NUM_ROWS]) {
    for i in 0..NUM_ROWS {
        for j in 0..NUM_COLS {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

// Return true if this series of squares contains at most one queen.
// The function starts at square [r0][c0] and enter a loop.
// Each time through the loop it adds dr to the row and dc to the column.
// It repeats the loop until the row or column falls off the board, counting the number of
// queens as it goes. If the function finds two or more queens, then it returns false.
pub fn series_is_legal(
    board: &mut [[&str; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut num_queens = 0;
    let mut r = r0;
    let mut c = c0;
    while r >= 0 && r < INUM_ROWS && c >= 0 && c < INUM_COLS && num_queens < 2 {
        if board[r as usize][c as usize] == QUEEN_SQUARE {
            num_queens += 1;
        }
        // Move to the next square in the series.
        r += dr;
        c += dc;
    }
    num_queens < 2
}

// Return true if the board is legal.
pub fn board_is_legal(board: &mut [[&str; NUM_COLS]; NUM_ROWS]) -> bool {
    // See if each row is legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 0, 1) {
            return false;
        }
    }

    // See if each column is legal.
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, 0) {
            return false;
        }
    }

    // See if diagonals down to the right are legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 1, 1) {
            return false;
        }
    }
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, 1) {
            return false;
        }
    }

    // See if diagonals down to the left are legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, INUM_ROWS - 1, 1, -1) {
            return false;
        }
    }
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, -1) {
            return false;
        }
    }

    // If we survived this long, then the board is legal.
    true
}

// Return true if the board is legal and a solution.
pub fn board_is_a_solution(board: &mut [[&str; NUM_COLS]; NUM_ROWS]) -> bool {
    // See if it is legal.
    if !board_is_legal(board) {
        return false;
    }

    // See if the board contains exactly NUM_ROWS queens.
    let mut num_queens = 0;
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            if board[r as usize][c as usize] == QUEEN_SQUARE {
                num_queens += 1;
            }
        }
    }
    num_queens == NUM_ROWS
}
