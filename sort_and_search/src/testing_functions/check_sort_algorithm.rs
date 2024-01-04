// Print at most num_items items.
pub fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

// Verify that the Vec is sorted.
pub fn check_sorted(vec: &Vec<i32>) -> bool {
    for i in 0..(vec.len() - 1) {
        if vec[i] > vec[i + 1] {
            println!("The vector is NOT sorted");
            return false;
        }
    }
    println!("The vector is sorted");
    true
}
