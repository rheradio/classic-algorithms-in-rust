use crate::n_queens::board_set_and_check::*;

// The board parameter is the current board arrangement that shows where any previously positioned
// queens are waiting to ambush new queens.
// The r and c values give the position that the function should examine.
// It will see what happens if we put a queen there or not. As you’ll see shortly, the function
// will recursively examine every square. After it has made an assignment for every square,
// we’ll check to see whether we have a solution.
pub fn place_queens_brute_force(board: &mut [[&str; NUM_COLS]; NUM_ROWS], r: i32, c: i32) -> bool {
    if r == INUM_ROWS {
        // Base case: we have finished examining every position and have
        // fallen off the board
        return board_is_a_solution(board);
    } else {
        // Recursion case: there are two options for square [r][c]:
        // it should either have a queen or not

        // Let's see what happens if the square doesn't have a queen
        board[r as usize][c as usize] = EMPTY_SQUARE;
        let (mut next_r, mut next_c) = (r, c);
        if c < (INUM_COLS - 1) {
            next_c += 1;
        } else {
            next_r += 1;
            next_c = 0;
        }
        if place_queens_brute_force(board, next_r, next_c) {
            return true;
        } else {
            // It didn't work. Let's see what happens if the square has a queen
            board[r as usize][c as usize] = QUEEN_SQUARE;
            let queen_try = place_queens_brute_force(board, next_r, next_c);
            if !queen_try {
                board[r as usize][c as usize] = EMPTY_SQUARE;
            }
            return queen_try;
        }
    }
}
