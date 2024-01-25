use crate::n_queens::board_set_and_check::*;
use crate::n_queens::place_queens_counting_attacks::adjust_attack_counts;

// The happy idea here is to reduce the number of variables we are making recursion:
// from two (rows and cols) to one (cols).
// Watch https://www.youtube.com/watch?v=Ph95IHmRp5M
pub fn place_queens_redux(board: &mut [[&str; NUM_COLS]; NUM_ROWS], c: i32) -> bool {
    let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];

    return place_queens_redux_inner(board, c, &mut num_attacking);

    pub fn place_queens_redux_inner(
        board: &mut [[&str; NUM_COLS]; NUM_ROWS],
        c: i32,
        num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    ) -> bool {
        if !board_is_legal(board) {
            // Base case: the board is illegal
            return false;
        } else {
            if c == INUM_COLS {
                // Base case: we have reach the end of the board
                return true;
            } else {
                // Recursion case
                for r in 0..INUM_ROWS {
                    if num_attacking[r as usize][c as usize] == 0 {
                        board[r as usize][c as usize] = QUEEN_SQUARE;
                        adjust_attack_counts(num_attacking, r, c, 1);
                        let queen_try = place_queens_redux_inner(board, c + 1, num_attacking);
                        if queen_try {
                            return true;
                        } else {
                            board[r as usize][c as usize] = EMPTY_SQUARE;
                            adjust_attack_counts(num_attacking, r, c, -1);
                        }
                    }
                }
                return false;
            }
        }
    }
}
