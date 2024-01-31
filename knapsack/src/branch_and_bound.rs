use crate::knapsack_random_generation::*;

// branch_and_bound  is a façade function for do_branch_and_bound
// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made
pub fn branch_and_bound(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    // (solution, total_value, function_calls)
    let next_index = 0;
    let mut best_value = 0;
    let current_value = 0;
    let current_weight = 0;
    let remaining_value = sum_values(items, true);
    return do_branch_and_bound(
        items,
        allowed_weight,
        next_index,
        &mut best_value,
        current_value,
        current_weight,
        remaining_value,
    );
    // output: (solution, total_value, function_calls)
}

fn do_branch_and_bound(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: usize,
    best_value: &mut i32,
    current_value: i32,
    current_weight: i32,
    remaining_value: i32,
) -> (Vec<Item>, i32, i32) {
    // Base case
    if next_index == items.len() {
        let copied_items = copy_items(items);
        let solution_value = solution_value(&copied_items, allowed_weight);
        if solution_value > *best_value {
            *best_value = solution_value;
        }
        let function_calls = 1;
        return (copied_items, solution_value, function_calls);
    }

    // Recursive case

    // The current branch cannot improve the best_value: stop
    if current_value + remaining_value <= *best_value {
    let empty_vec: Vec<Item> = vec![];
        let solution_value = 0;
        let function_calls = 1;
        return (empty_vec, solution_value, function_calls);
    }

    let mut solution_value_select = 0;
    let mut items_select: Vec<Item> = vec![];
    let mut fc_selected = 1;
    if current_weight + items[next_index].weight <= allowed_weight {
        items[next_index].is_selected = true;
        (items_select, solution_value_select, fc_selected) = do_branch_and_bound(
            items,
            allowed_weight,
            next_index + 1,
            best_value,
            current_value + items[next_index].value,
            current_weight + items[next_index].weight,
            remaining_value - items[next_index].value,
        );
    }

    items[next_index].is_selected = false;
    let (items_unselect, solution_value_unselect, fc_unselected) = do_branch_and_bound(
        items,
        allowed_weight,
        next_index + 1,
        best_value,
        current_value,
        current_weight,
        remaining_value - items[next_index].value,
    );

    let function_calls = 1 + //myself
            fc_selected + fc_unselected;

    if solution_value_select > solution_value_unselect {
        return (items_select, solution_value_select, function_calls);
    } else {
        return (items_unselect, solution_value_unselect, function_calls);
    }
}
