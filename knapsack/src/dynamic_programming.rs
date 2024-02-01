use crate::knapsack_random_generation::*;

fn print_matrix(m: &Vec<Vec<i32>>) {
    println!("\n=================================================================================");
    for i in 0..m[0].len() {
        print!("{:2} ", i);
    }
    println!("\n=================================================================================");
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            print!("{:2} ", m[i][j]);
        }
        println!();
    }
}

// Use dynamic programming to find a solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
pub fn dynamic_programming(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    let mut solution_value: Vec<Vec<i32>> =
        vec![vec![0; allowed_weight as usize + 1]; NUM_ITEMS_USIZE];
    let mut prev_weight: Vec<Vec<i32>> =
        vec![vec![0; allowed_weight as usize + 1]; NUM_ITEMS_USIZE];

    // Initialize row 0
    for w in 0..=allowed_weight as usize {
        if items[0].weight > w as i32 {
            // solution_value[0][w] = 0;
            prev_weight[0][w] = w as i32;
        } else {
            solution_value[0][w] = items[0].value;
            prev_weight[0][w] = -1;
        }
    }

    // Fill in the remaining table rows.
    for i in 1..NUM_ITEMS_USIZE {
        for w in 0..=allowed_weight as usize {
            if items[i].weight as usize <= w {
                if solution_value[i - 1][w]
                    > (solution_value[i - 1][w - items[i].weight as usize] + items[i].value)
                {
                    solution_value[i][w] = solution_value[i - 1][w];
                    prev_weight[i][w] = w as i32;
                } else {
                    solution_value[i][w] =
                        solution_value[i - 1][w - items[i].weight as usize] + items[i].value;
                    prev_weight[i][w] = w as i32 - items[i].weight;
                }
            } else {
                solution_value[i][w] = solution_value[i - 1][w];
                prev_weight[i][w] = w as i32;
            }
        }
    }

    for item in &mut *items {
        item.is_selected = false;
    }

    let mut back_i = NUM_ITEMS_USIZE - 1;
    let mut back_w = allowed_weight as usize;
    while back_i >= 0 {
        if prev_weight[back_i][back_w] != back_w as i32 {
            items[back_i].is_selected = true;
            back_w = prev_weight[back_i][back_w] as usize;
        }
        match back_i {
            0 => break,
            _ => back_i -= 1
        };
    }

    let copied_items = copy_items(items);
    let solution_value = solution_value[NUM_ITEMS_USIZE - 1][allowed_weight as usize];
    let function_calls = 1;
    return (copied_items, solution_value, function_calls);
}
