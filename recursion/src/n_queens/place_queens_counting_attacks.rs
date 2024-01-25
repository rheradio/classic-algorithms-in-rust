use crate::n_queens::board_set_and_check::*;

// Add amount to the attack counts for this square.
fn adjust_attack_counts(num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS], r: i32, c: i32, amount: i32) {
    // Attacks in the same row.
    for i in 0..INUM_COLS {
        num_attacking[r as usize][i as usize] += amount
    }

    // Attacks in the same column.
    for i in 0..INUM_ROWS {
        num_attacking[i as usize][c as usize] += amount
    }

    // Attacks in the upper left to lower right diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = r + i;
        let test_c = c + i;
        if test_r >= 0 && test_r < INUM_ROWS &&
            test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }

    // Attacks in the upper right to lower left diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = r + i;
        let test_c = c - i;
        if test_r >= 0 && test_r < INUM_ROWS &&
            test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }
}

// The board parameter is the current board arrangement that shows where any previously positioned
// queens are waiting to ambush new queens.
// The r and c values give the position that the function should examine.
// It will see what happens if we put a queen there or not. As you’ll see shortly, the function
// will recursively examine every square. After it has made an assignment for every square,
// we’ll check to see whether we have a solution.
pub fn place_queens_counting_attacks(
    board: &mut [[&str; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
) -> bool {

    // Make a num_attacking matrix.
    // The value num_attacking[r as usize][c as usize] is the number
    // of queens that can attack square (r, c).
    let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];

    return place_queens_counting_attacks_inner(board, r, c, 0, &mut num_attacking);

    pub fn place_queens_counting_attacks_inner(
        board: &mut [[&str; NUM_COLS]; NUM_ROWS],
        r: i32,
        c: i32,
        queens_placed: i32,
        num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    ) -> bool {
        if queens_placed == INUM_ROWS {
            // Base case: we have finished to place all queens
            // Check if this is a solution
            return board_is_a_solution(board);
        } else if r == INUM_ROWS {
            // Base case: we have finished examining every position and have
            // fallen off the board
            return false;
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

            if place_queens_counting_attacks_inner(
                board,
                next_r,
                next_c,
                queens_placed,
                num_attacking,
            ) {
                return true;
            } else {
                // It didn't work. Let's see what happens if the square has a queen

                if num_attacking[r as usize][c as usize] == 0 {
                    board[r as usize][c as usize] = QUEEN_SQUARE;
                    adjust_attack_counts(num_attacking, r, c, 1);
                    let queen_try = place_queens_counting_attacks_inner(
                        board,
                        next_r,
                        next_c,
                        queens_placed + 1,
                        num_attacking,
                    );
                    if !queen_try {
                        board[r as usize][c as usize] = EMPTY_SQUARE;
                        adjust_attack_counts(num_attacking, r, c, -1);
                    }
                    return queen_try;
                } else {
                    return false
                }
            }
        } // Recursion case
    } // inner
}
