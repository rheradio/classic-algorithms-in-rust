#[cfg(test)]
mod test {
    use crate::n_queens::board_set_and_check::*;
    use crate::n_queens::place_queens_brute_force::place_queens_brute_force;
    use std::time::Instant;

    #[test]
    fn test_n_queens() {
        // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
        let mut board = [[EMPTY_SQUARE; NUM_COLS]; NUM_ROWS];

        let start = Instant::now();
        let success = place_queens_brute_force(&mut board, 0, 0);
        //let success = place_queens_2(& mut board, 0, 0, 0);
        //let success = place_queens_3(& mut board);
        let duration = start.elapsed();

        println!("Time: {:?}", duration);

        if success {
            println!("Success!");
        } else {
            println!("Failure! I couldn't place {} queen on the board.", NUM_ROWS);
        }

        dump_board(&mut board);
    }
}
