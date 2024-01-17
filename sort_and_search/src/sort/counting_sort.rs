pub fn counting_sort(vec: &mut Vec<i32>) {

    // Make a copy of vec and get its max value
    let n = vec.len();
    let mut aux_vec: Vec<i32> = vec![0; n];
    let mut max = 0;
    for i in 0..n {
        aux_vec[i] = vec[i];
        if max < vec[i] {
            max = vec[i];
        }
    }
    max += 1;

    // Count the number of occurrences of each digit in aux_vec
    let mut counts = vec![0; max as usize];
    for i in 0..n {
        counts[aux_vec[i] as usize] += 1;
    }
    // counts[i] is the value of the number of elements in aux_vec equal to i

    // Change counts to show the cumulative number of digit occurrences
    for i in 1..max as usize {
        counts[i] = counts[i] + counts[i-1];
    }
    // Counts now have the number of elements <= i

    // Go through aux_vec backwards and update vec
    let mut i = n-1;
    loop {
        let value = aux_vec[i] as usize;
        if counts[value] > 0 {
            counts[value] -= 1;
        }
        vec[counts[value]] = aux_vec[i];
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::testing_functions::generate_random_vector::make_random_vec;
    use super::*;
    use crate::testing_functions::check_sort_algorithm::check_sorted;

    #[test]
    fn test_counting_sort() {
        let num_items = 100;
        let max = 50;
        let mut vec = make_random_vec(num_items, max);
        counting_sort(&mut vec);
        assert!(check_sorted(&vec));
    }
}
