use std::time::{SystemTime, UNIX_EPOCH};
pub const NUM_ITEMS: i32 = 20; // A reasonable value for exhaustive search.
pub const MIN_VALUE: i32 = 1;
pub const MAX_VALUE: i32 = 10;
pub const MIN_WEIGHT: i32 = 4;
pub const MAX_WEIGHT: i32 = 10;

pub struct Item {
    pub value: i32,
    pub weight: i32,
    pub is_selected: bool,
}

// ************
// *** Prng ***
// ************
// The Prng struct represents a pseudorandom number generator.
// It has a seed value as its sole field
pub struct Prng {
    pub seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}

// Make some random items.
// This function creates a vector to hold the new items.
// Then for each item, it generates a random value between
// min_value and max_value. Similarly, it generates a random
// weight between min_weight and max_weight. It uses those
// values to create a new Item struct, setting its is_selected
// field to false. It then pushes the new Item onto the end of
// the vector
pub fn make_items(
    prng: &mut Prng,
    num_items: i32,
    min_value: i32,
    max_value: i32,
    min_weight: i32,
    max_weight: i32,
) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        let item = Item {
            value: prng.next_i32(min_value, max_value),
            weight: prng.next_i32(min_weight, max_weight),
            is_selected: false,
        };
        items.push(item);
    }
    return items;
}

// Return a copy of the items.
// The copy_items function makes a copy of a vector of items and
// returns it. We use this so each algorithm can have its own copy
// of the items without messing up the vector for any later algorithms
pub fn copy_items(items: &Vec<Item>) -> Vec<Item> {
    let mut new_items: Vec<Item> = Vec::with_capacity(items.len());
    for item in items {
        let new_item = Item {
            value: item.value,
            weight: item.weight,
            is_selected: item.is_selected,
        };
        new_items.push(new_item);
    }
    return new_items;
}

// Return the total value of the items.
// If add_all is true, add up all items.
// If add_all is false, only add up the selected items.
pub fn sum_values(items: &Vec<Item>, add_all: bool) -> i32 {
    if add_all {
        return items.iter().map(|item| item.value).sum();
    } else {
        return items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.value)
            .sum();
    }
}

// Return the total weight of the items.
// If add_all is false, only add up the selected items.
// If add_all is true, add up all items.
pub fn sum_weights(items: &Vec<Item>, add_all: bool) -> i32 {
    if add_all {
        return items.iter().map(|item| item.weight).sum();
    } else {
        return items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.weight)
            .sum();
    }
}

// Return the value of this solution.
// If the solution is too heavy, return -1 so we prefer an empty solution.
pub fn solution_value(items: &Vec<Item>, allowed_weight: i32) -> i32 {
    // If the solution's total weight > allowed_weight,
    // return -1 so even an empty solution is better.
    if sum_weights(items, false) > allowed_weight {
        return -1;
    }

    // Return the sum of the selected values.
    return sum_values(items, false);
}

// Print the selected items.
// If add_all is true, print all items
pub fn print_items(items: &Vec<Item>, all: bool) {
    let mut num_printed = 0;
    println!("==================================================");
    println!("= Items ==========================================");
    println!("==================================================");
    for i in 0..items.len() {
        if items[i].is_selected || all {
            println!(
                "item {}: value = {}, weight = {}, is_selected = {}",
                i, items[i].value, items[i].weight, items[i].is_selected
            );
        }
        num_printed += 1;
        if num_printed > 100 {
            println!("...");
            return;
        }
    }
    println!("==================================================");
}
