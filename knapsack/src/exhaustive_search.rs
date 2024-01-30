use crate::knapsack_random_generation::*;

// exhaustive_search is a façade function for do_exhaustive_search
// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made
pub fn exhaustive_search(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    return do_exhaustive_search(items, allowed_weight, 0 as usize);
    // output: (solution, total_value, function_calls)
}

fn do_exhaustive_search(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: usize,
) -> (Vec<Item>, i32, i32) {
    if next_index == items.len() {
        // Base case
        let copied_items = copy_items(items);
        let solution_value = solution_value(&copied_items, allowed_weight);
        let function_calls = 1;
        return (copied_items, solution_value, function_calls);
    } else {
        // Recursive case
        items[next_index].is_selected = true;
        let (items_select, solution_value_select, fc_selected) =
            do_exhaustive_search(items, allowed_weight, next_index + 1);
        items[next_index].is_selected = false;
        let (items_unselect, solution_value_unselect, fc_unselected) =
            do_exhaustive_search(items, allowed_weight, next_index + 1);
        let function_calls = 1 + //myself
            fc_selected + fc_unselected;
        if solution_value_select > solution_value_unselect {
            return (items_select, solution_value_select, function_calls);
        } else {
            return (items_unselect, solution_value_unselect, function_calls);
        }
    }
}
