// O(log_2 n)
// Perform binary search.
// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn binary_search(vec: &[i32], target: i32) -> (i32, i32) {
    let mut low = 0;
    let mut high = vec.len() - 1;
    let mut num_tests = 0;
    while low <= high {
        num_tests += 1;
        let mid = (low + high) / 2;
        let guess = vec[mid];
        if guess == target {
            return (mid.try_into().unwrap(), num_tests);
        } else if guess > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    (-1, num_tests)
}

#[cfg(test)]
use std::collections::HashSet;
mod test {
    use super::*;
    use crate::sort::bubble_sort::bubble_sort;
    use crate::testing_functions::generate_random_vector::make_random_vec;

    #[test]
    fn test_binary_search() {
        let num_items = 100;
        let max = 500;
        let vec = make_random_vec(num_items, max);
        // remove duplicated items
        let mut vec: Vec<i32> = vec
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        // VERY IMPORTANT: sort vec
        bubble_sort(&mut vec);

        // An item inside the vector.
        let pos = 50;
        let item = vec[pos];
        let (found_pos, num_tests) = binary_search(&mut vec, item);
        let pos_as_i32: i32 = pos.try_into().unwrap();
        assert_eq!(found_pos, pos_as_i32);
        let max_predicted_tests: i32 = (vec.len() as f32).log2().ceil() as i32;
        assert!(num_tests <= max_predicted_tests);

        // An item outside the vector.
        let item = max + 1;
        let (found_pos, num_tests) = binary_search(&mut vec, item);
        assert_eq!(found_pos, -1);
        assert!(num_tests <= max_predicted_tests);
    }
}
