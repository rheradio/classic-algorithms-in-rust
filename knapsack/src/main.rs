mod knapsack_random_generation;
use crate::knapsack_random_generation::*;

mod branch_and_bound;
mod exhaustive_search;

use crate::branch_and_bound::branch_and_bound;
use crate::exhaustive_search::exhaustive_search;

use std::time::Instant;

// Run the algorithm. Display the elapsed time and solution.
pub fn run_algorithm(
    alg: &dyn Fn(&mut Vec<Item>, i32) -> (Vec<Item>, i32, i32),
    items: &mut Vec<Item>,
    allowed_weight: i32,
) {
    // Copy the items so the run isn't influenced by a previous run.
    let mut test_items = copy_items(items);

    let start = Instant::now();

    // Run the algorithm.
    let mut solution: Vec<Item>;
    let total_value: i32;
    let function_calls: i32;
    (solution, total_value, function_calls) = alg(&mut test_items, allowed_weight);

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    print_items(&mut solution, false);
    println!(
        "Total value: {}, Total weight: {}, #Calls: {}",
        total_value,
        sum_weights(&mut solution, false),
        function_calls
    );
    println!();
}

fn main() {
    // Prepare a Prng using the same seed each time.
    let mut prng = Prng { seed: 1337 };
    //prng.randomize();

    // Make some random items.
    let mut items = make_items(
        &mut prng, NUM_ITEMS, MIN_VALUE, MAX_VALUE, MIN_WEIGHT, MAX_WEIGHT,
    );
    let allowed_weight = sum_weights(&mut items, true) / 2;

    // Display basic parameters.
    println!("*** Parameters ***");
    println!("# items:        {}", NUM_ITEMS);
    println!("Total value:    {}", sum_values(&mut items, true));
    println!("Total weight:   {}", sum_weights(&mut items, true));
    println!("Allowed weight: {}", allowed_weight);
    println!();

    print_items(&items, true);

    // Exhaustive search
    if NUM_ITEMS > 23 {
        // Only run exhaustive search if num_items is small enough.
        println!("Too many items for exhaustive search\n");
    } else {
        println!("*** Exhaustive Search ***");
        run_algorithm(&exhaustive_search, &mut items, allowed_weight);
    }

    // // Exhaustive search
    if NUM_ITEMS > 23 {
        // Only run exhaustive search if num_items is small enough.
        println!("Too many items for branch and boud\n");
    } else {
        println!("*** Branch & Bound ***");
        run_algorithm(&branch_and_bound, &mut items, allowed_weight);
    }
}
