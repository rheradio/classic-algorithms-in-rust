// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    for (pos, item) in vec.iter().enumerate() {
        if *item == target {
            let pos: i32 = pos.try_into().unwrap();
            return (pos, pos + 1);
        }
    }
    (-1, vec.len().try_into().unwrap())
}

#[cfg(test)]
use std::collections::HashSet;
mod test {
    use super::*;
    use crate::testing_functions::generate_random_vector::make_random_vec;

    #[test]
    fn test_linear_search() {
        let num_items = 100;
        let max = 500;
        let vec = make_random_vec(num_items, max);
        // remove duplicated items
        let vec: Vec<i32> = vec
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        // An item inside the vector.
        let pos = 50;
        let item = vec[pos];
        let (found_pos, num_tests) = linear_search(&vec, item);
        let pos_as_i32 = pos.try_into().unwrap();
        assert_eq!(found_pos, pos_as_i32);
        assert_eq!(num_tests, pos_as_i32 + 1);

        // An item outside the vector.
        let item = max + 1;
        let (found_pos, num_tests) = linear_search(&vec, item);
        assert_eq!(found_pos, -1);
        assert_eq!(num_tests, vec.len().try_into().unwrap());
    }
}
