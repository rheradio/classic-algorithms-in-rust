mod generate_random_vector;
use generate_random_vector::{get_i32, make_random_vec};

mod test_algorithm;
use test_algorithm::{check_sorted, print_vec};

mod bubble_sort;
use bubble_sort::bubble_sort;

fn main() {
    let num_items = get_i32("Number of items?");
    let max = get_i32("Max value?");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    bubble_sort(&mut vec);
    print_vec(&vec, num_items);
    check_sorted(&vec);
}
