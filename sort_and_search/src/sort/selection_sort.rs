// See Chapter 2 in Aditya Y Bhargava. Grokking Algorithms: An illustrated guide for programmers
// and other curious people. Manning Publications Co., 2nd. Ed., 2024.
// O(n^2)
pub fn selection_sort(vec: &mut Vec<i32>) {
    let n = vec.len();
    let mut min_pos: usize;
    for i in 0..n - 1 {
        min_pos = i;
        for j in (i + 1)..n {
            if vec[j] < vec[min_pos] {
                min_pos = j;
            }
        }
        vec.swap(i, min_pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sort::selection_sort::selection_sort;
    use crate::testing_functions::check_sort_algorithm::{check_sorted, print_vec};
    use crate::testing_functions::generate_random_vector::make_random_vec;

    #[test]
    fn test_selection_sort() {
        let num_items = 100;
        let max = 500;
        let mut vec = make_random_vec(num_items, max);
        selection_sort(&mut vec);
        assert!(check_sorted(&vec));
    }
}
