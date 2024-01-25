// To measure stack consumption:
// https://github.com/d99kris/stackusage
// Once installed, run:
// cargo build --release
// stackusage target/release/recursion

mod factorial;
mod fibonacci;
mod knights_tour;
mod n_queens;

use factorial::factorial::factorial;
use factorial::factorial_tail_recursion::factorial_tail_recursion;

fn main() {
    let mut acc: u128 = 0;
    for _ in 0..10000 {
        for i in 0..35 {
            //acc = acc + factorial(i);
            acc = acc + factorial_tail_recursion(i);
        }
    }
    println!("acc: {}", acc);
}

//////////////////////////////////////////////////////
// factorial
//////////////////////////////////////////////////////
// pid  id    tid  requested     actual     maxuse  max%    dur               funcP name
// 13532   0  13532    8388608    8376320       4328     0      0               (nil) recursion
// ////////////////////////////////////////////////////

//////////////////////////////////////////////////////
// factorial_tail_recursion
//////////////////////////////////////////////////////
// pid  id    tid  requested     actual     maxuse  max%    dur               funcP name
// 13770   0  13770    8388608    8376320       4712     0      0               (nil) recursion
// ////////////////////////////////////////////////////

// No difference in stack consumption between the two factorial implementations!
