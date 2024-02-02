mod breadth_first;
mod depth_first;

use crate::breadth_first::breadth_first;
use crate::depth_first::depth_first;
use std::collections::HashMap;

fn main() {
    let mut graph: HashMap<String, Option<Vec<String>>> = HashMap::new();
    graph.insert(
        String::from("you"),
        Some(vec![
            String::from("alice"),
            String::from("bob"),
            String::from("claire"),
        ]),
    );
    graph.insert(
        String::from("bob"),
        Some(vec![String::from("anuj"), String::from("peggy")]),
    );
    graph.insert(String::from("alice"), Some(vec![String::from("peggy")]));
    graph.insert(
        String::from("claire"),
        Some(vec![String::from("thom"), String::from("jonny")]),
    );
    graph.insert(String::from("anuj"), None);
    graph.insert(String::from("peggy"), None);
    graph.insert(String::from("thom"), None);
    graph.insert(String::from("jonny"), None);

    println!("# Breadth-First Search ###########");
    breadth_first(&graph, &String::from("you"));

    println!("# Depth-First Search ###########");
    depth_first(&graph, &String::from("you"));
}
