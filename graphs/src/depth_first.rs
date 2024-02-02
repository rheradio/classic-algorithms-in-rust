use std::collections::HashMap;

pub fn depth_first(graph: &HashMap<String, Option<Vec<String>>>, start_node: &String) {
    let mut searched: Vec<String> = vec![];
    depth_first_inner(graph, start_node, &mut searched);

    fn depth_first_inner(
        graph: &HashMap<String, Option<Vec<String>>>,
        start_node: &String,
        searched: &mut Vec<String>,
    ) {
        let nodes = graph.get(start_node).unwrap();
        match nodes {
            None => {} // Base case
            Some(vector) => {
                // Recursive case
                for node in vector {
                    let already_searched = searched.iter().any(|i| i == node);
                    if !already_searched {
                        searched.push((*node).clone());
                        depth_first_inner(graph, node, searched);
                        println!("{}", node);
                    }
                }
            }
        }
    }
}
