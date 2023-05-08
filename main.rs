use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "soc-Epinions1.txt";
    let adj_list = read_graph(path);
    let avg_dist = compute_avg_shortest_path_length(&adj_list).unwrap_or(-1.0);
    let diameter = compute_diameter(&adj_list).unwrap_or(u32::MAX);
    println!("The average distance between pairs of vertices is: {}", avg_dist);
    println!("The diameter of the graph is: {}", diameter);
}

// Define a function to read in a directed graph from a file and construct an adjacency list representation
fn read_graph(file_path: &str) -> HashMap<u32, Vec<u32>> {
    let file = File::open(file_path).expect("failed to open file");
    let reader = BufReader::new(file);

    let mut adj_list = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        if line.starts_with('#') {
            continue;
        }
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let from_node = split[0].parse::<u32>().expect("failed to parse node ID");
        let to_node = split[1].parse::<u32>().expect("failed to parse node ID");
        let neighbors = adj_list.entry(from_node).or_insert(Vec::new());
        neighbors.push(to_node);
    }

    adj_list
}

// Define a function to compute the average shortest path length between all pairs of nodes in a graph using BFS
fn compute_avg_shortest_path_length(adj_list: &HashMap<u32, Vec<u32>>) -> Option<f32> {
    let mut total_shortest_path_length = 0;
    let mut num_pairs_processed = 0;

    // Iterate over every pair of nodes in the graph
    for &start_node in adj_list.keys() {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = Vec::new();

        // Initialize the BFS algorithm with the starting node
        visited.insert(start_node);
        queue.push(start_node);
        distances.insert(start_node, 0);

        // Perform the BFS algorithm to compute shortest path lengths to all other nodes
        while !queue.is_empty() {
            let current_node = queue.remove(0);
            let current_distance = distances[&current_node];

            for &neighbor in &adj_list[&current_node] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }

        // Compute the total shortest path length from the starting node to all other nodes
        for (&end_node, &distance) in &distances {
            if start_node != end_node {
                total_shortest_path_length += distance;
                num_pairs_processed += 1;
            }
        }
    }

    // If no pairs were processed, return None to avoid division by zero
    if num_pairs_processed == 0 {
        None
    } else {
        Some(total_shortest_path_length as f32 / num_pairs_processed as f32)
    }
} 


// Define a function to compute the diameter of a graph
fn compute_diameter(adj_list: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    let mut max_distance = 0;

    // Iterate over every node in the graph and compute the shortest path lengths to all other nodes
    for &start_node in adj_list.keys() {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();

        // Initialize the BFS algorithm with the starting node
        visited.insert(start_node);
        queue.push((start_node, 0));

        // Perform the BFS algorithm to compute shortest path lengths to all other nodes
        while !queue.is_empty() {
            let (current_node, current_distance) = queue.remove(0);

            // Update the maximum distance seen so far
            if current_distance > max_distance {
                max_distance = current_distance;
            }

            for &neighbor in &adj_list[&current_node] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push((neighbor, current_distance + 1));
                }
            }
        }
    }

    // If max_distance is still 0, then the graph is disconnected and has no diameter
    if max_distance == 0 {
        None
    } else {
        Some(max_distance)
    }

// some tests   
}
fn test_compute_avg_shortest_path_length(){
    let path = "soc-Epinions1.txt";
    let adj_list = read_graph(path);
    let avg_dist = compute_avg_shortest_path_length(&adj_list).unwrap_or(-1.0);
    assert_eq!(avg_dist, 3.0);

} 
fn test_compute_diameter(){
    let path = "soc-Epinions1.txt";
    let adj_list = read_graph(path);
    let diameter = compute_diameter(&adj_list).unwrap_or(u32::MAX);
    assert_eq!(diameter, 9);
}
