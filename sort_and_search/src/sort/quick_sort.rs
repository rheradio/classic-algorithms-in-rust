// Lomuto partition scheme
// https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme
fn partition(vec: &mut [i32], low: usize, high: usize) -> usize {
    // Choose the last element as the pivot
    let pivot: i32 = vec[high];

    // Temporary pivot index
    let mut i = low;

    for j in low..high {
        if vec[j] <= pivot {
            // Swap the current element with the element at the temporary pivot index
            vec.swap(i, j);
            // Move the temporary pivot index forward
            i += 1;
        }
    }
    vec.swap(i, high);
    i
}

pub fn quick_sort(vec: &mut Vec<i32>) {
    let length = vec.len();
    quick_sort_inner(vec, 0, length - 1);
    pub fn quick_sort_inner(vec: &mut Vec<i32>, low: usize, high: usize) {
        if vec.len() > 2 {
            let pivot_index = partition(vec, low, high);
            if low != pivot_index {
                quick_sort_inner(vec, low, pivot_index - 1);
            }
            if high != pivot_index {
                quick_sort_inner(vec, pivot_index + 1, high);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_functions::check_sort_algorithm::{check_sorted, print_vec};
    use crate::testing_functions::generate_random_vector::make_random_vec;

    #[test]
    fn test_partition() {
        let num_items = 100;
        let max = 500;
        let mut vec = make_random_vec(num_items, max);

        let length = vec.len();
        let pivot_index = partition(&mut vec, 0, length - 1);

        for i in 0..pivot_index {
            assert!(vec[i] <= vec[pivot_index]);
        }
        for i in pivot_index + 1..length {
            assert!(vec[i] >= vec[pivot_index]);
        }
    }

    #[test]
    fn test_quick_sort() {
        let num_items = 100;
        let max = 500;
        let mut vec = make_random_vec(num_items, max);
        quick_sort(&mut vec);
        assert!(check_sorted(&vec));
    }
}
