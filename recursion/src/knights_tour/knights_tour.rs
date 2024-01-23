use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 5;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Board start position
const START_POSITION: (usize, usize) = (2, 2);

#[derive(Copy, Clone)]
pub enum Square {
    Unvisited,
    Visited(i32)
}
use Square::*;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Function that takes a board array as input and prints the board.
pub fn dump_board(board: &mut [[Square;NUM_COLS]; NUM_ROWS]) {
    for i in 0..NUM_ROWS {
        for j in 0..NUM_COLS {
            match board[i][j] {
                // Check https://doc.rust-lang.org/std/fmt/index.html#usage for formatting options.
                Visited(n) => print!("{:3}",n),
                Unvisited => print!("  □")
            };
        }
        println!();
    }
}

// Function that tries to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(board: &mut [[Square; NUM_COLS]; NUM_ROWS],
             offsets: &[(i32, i32); 8],    // 8 possible moves, 2 coordinates each.
             cur_row: usize, cur_col: usize,
             num_visited: i32) -> bool {

    let (mut next_row, mut next_col): (i32, i32);

    if num_visited == INUM_ROWS*INUM_COLS { // Base case
        if REQUIRE_CLOSED_TOUR {
            // Check if any of the allowed movements place us at the
            // starting point (0, 0);
            for (x, y) in offsets {
                (next_row, next_col) = (cur_row as i32 + *x, cur_row as i32 + *y);
                if (next_row, next_col) == (0, 0) {
                    return true
                }
            }
            return false;
        } else {
            // If we allow open tours, we have reached a valid solution.
            return true
        }
    } else { // Recursive case
        for (x, y) in offsets {
            let (next_row, next_col): (i32, i32) = (cur_row as i32 + *x, cur_col as i32 + *y);
            if next_row >=0 && next_row < INUM_ROWS && next_col >=0 && next_col < INUM_COLS {
                let (next_row, next_col): (usize, usize) = (next_row as usize, next_col as usize);
                if let Unvisited = board[next_row][next_col] {
                    board[next_row][next_col] = Visited(num_visited);
                    if find_tour(board, offsets, next_row, next_col, num_visited + 1) {
                        return true
                    } else {
                        board[next_row][next_col] = Unvisited;
                    }
                }
            }
        }
        return false
    }
}

#[cfg(test)]
mod test {
    use Square::Unvisited;
    use super::*;

    #[test]
    fn test_knights_tour() {

        // Initialize the vector with the knight’s legal moves.
        // For example, the first entry (-2, -1) means the knight can move -2 rows (2 rows up) and
        // -1 column (1 column left) from its current position. Because the moves are relative to
        // the knight’s current position, some of the moves may not be allowed at a given moment,
        // either because the knight has already visited that square or because the knight is near
        // the edge and the move is off the board.
        let offsets = [
            (-2, -1),
            (-1, -2),
            ( 2, -1),
            ( 1, -2),
            (-2,  1),
            (-1,  2),
            ( 2,  1),
            ( 1,  2),
        ];

        // Try to find a tour.
        let start = Instant::now();

        // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
        let mut board = [[Unvisited; NUM_COLS]; NUM_ROWS];

        // Start at board[0][0].
        board[START_POSITION.0][START_POSITION.1] = Visited(0);

        let mut number_of_tries = 1;
        loop {
            println!("Try #{number_of_tries}");
            if find_tour(&mut board, &offsets, START_POSITION.0, START_POSITION.1, 1) {
                break
            } else {
                let mut board = [[Unvisited; NUM_COLS]; NUM_ROWS];
                board[0][0] = Visited(0);
                number_of_tries += 1;
            }
        }
        let duration = start.elapsed();
        println!("Time: {:?}", duration);

        dump_board(&mut board);
    }
}

