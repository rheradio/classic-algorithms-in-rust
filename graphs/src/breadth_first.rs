use std::collections::HashMap;
use std::collections::VecDeque;

fn push_nodes<'a>(queue: &mut VecDeque<&'a String>, nodes: &'a Option<Vec<String>>) {
    match nodes {
        None => {}
        Some(v) => {
            for e in v {
                queue.push_back(e);
            }
        }
    }
}
pub fn breadth_first(graph: &HashMap<String, Option<Vec<String>>>, start_node: &String) {
    let mut queue: VecDeque<&String> = VecDeque::new();
    let mut searched: Vec<&String> = vec![];
    println!("{}", start_node);
    let mut nodes = graph.get(start_node).unwrap();
    push_nodes(&mut queue, nodes);
    let mut node: &String;
    while !queue.is_empty() {
        node = queue.pop_front().unwrap();
        println!("{}", node);
        if !searched.iter().any(|&i| i == node) {
            nodes = graph.get(node).unwrap();
            push_nodes(&mut queue, nodes);
            searched.push(node);
        }
    }
}
