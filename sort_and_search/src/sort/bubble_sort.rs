// O(n^2)
// Watch https://www.youtube.com/watch?v=M4ubFru2O80

pub fn bubble_sort(vec: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..(vec.len() - 1) {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
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
    fn test_bubble_sort() {
        let num_items = 100;
        let max = 500;
        let mut vec = make_random_vec(num_items, max);
        bubble_sort(&mut vec);
        assert!(check_sorted(&vec));
    }
}
