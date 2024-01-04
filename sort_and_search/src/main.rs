mod sort;
use sort::bubble_sort::bubble_sort;

mod testing_functions;
use testing_functions::generate_random_vector::{get_i32, make_random_vec};
use testing_functions::check_sort_algorithm::{check_sorted, print_vec};

fn main() {
    let num_items = get_i32("Number of items?");
    let max = get_i32("Max value?");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    bubble_sort(&mut vec);
    print_vec(&vec, num_items);
    check_sorted(&vec);
}
