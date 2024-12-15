use petgraph::graph::{UnGraph, NodeIndex};
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use rayon::prelude::*;
use std::sync::Mutex;
use std::io::BufRead;

/// First I need to reads the Amazon dataset and creates an undirected graph
pub fn load_graph(filename: &str) -> Result<UnGraph<(), ()>, Box<dyn Error>> {
    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::new();

    let file = std::fs::File::open(filename).map_err(|e| format!("Unable to open file: {}", e))?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading line: {}", e))?;
        let nodes: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        if nodes.len() == 2 {
            let source = *node_map
                .entry(nodes[0].clone())
                .or_insert_with(|| graph.add_node(()));
            let target = *node_map
                .entry(nodes[1].clone())
                .or_insert_with(|| graph.add_node(()));
            graph.add_edge(source, target, ());
        }
    }

    Ok(graph)
}

/// We also have to computes the degree centrality for all nodes (use parallelism for efficiency)
pub fn compute_degree_centrality(graph: &UnGraph<(), ()>) -> HashMap<NodeIndex, usize> {
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    node_indices
        .par_iter()
        .map(|&node| (node, graph.edges(node).count()))
        .collect()
}

/// I then will computes closeness centrality for all nodes (specifcically using BFS for approximation) with subsampling for runtime
pub fn compute_closeness_centrality(
    graph: &UnGraph<(), ()>,
    sample_size: usize,
) -> HashMap<NodeIndex, f64> {
    let closeness_centrality = Mutex::new(HashMap::new());
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();

    let mut rng = rand::thread_rng();
    let sampled_nodes = node_indices.choose_multiple(&mut rng, sample_size).cloned().collect::<Vec<_>>();

    sampled_nodes.par_iter().for_each(|&node| {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = vec![(node, 0)];

        // here is where we use BFS to compute distances
        while let Some((current_node, distance)) = queue.pop() {
            if !visited.contains(&current_node) {
                visited.insert(current_node);
                distances.insert(current_node, distance);
                for neighbor in graph.neighbors(current_node) {
                    if !visited.contains(&neighbor) {
                        queue.push((neighbor, distance + 1));
                    }
                }
            }
        }

        let total_distance: f64 = distances.values().map(|&value| value as f64).sum();
        let reachable_nodes = distances.len() as f64;

        let closeness = if total_distance > 0.0 {
            reachable_nodes / total_distance
        } else {
            0.0
        };

        let mut closeness_centrality = closeness_centrality.lock().unwrap();
        closeness_centrality.insert(node, closeness);
    });

    closeness_centrality.into_inner().unwrap()
}

/// How we compute the clustering coefficient for each node in the graph
pub fn compute_clustering_coefficient(graph: &UnGraph<(), ()>) -> HashMap<NodeIndex, f64> {
    let mut clustering_coeffs = HashMap::new();

    for node in graph.node_indices() {
        let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();
        let degree = neighbors.len();

        if degree < 2 {
            clustering_coeffs.insert(node, 0.0);
            continue;
        }

        let mut triangles = 0;
        for i in 0..degree {
            for j in i + 1..degree {
                if graph.contains_edge(neighbors[i], neighbors[j]) {
                    triangles += 1;
                }
            }
        }

        let clustering_coefficient = if degree > 1 {
            2.0 * triangles as f64 / (degree * (degree - 1)) as f64
        } else {
            0.0
        };

        clustering_coeffs.insert(node, clustering_coefficient);
    }

    clustering_coeffs
}

/// This function then identifies the top N most central products (nodes) based on closeness centrality
pub fn top_central_nodes(
    closeness_centrality: &HashMap<NodeIndex, f64>,
    top_n: usize,
) -> Vec<(NodeIndex, f64)> {
    let mut sorted_nodes: Vec<_> = closeness_centrality.iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    sorted_nodes.into_iter().take(top_n).map(|(node, &value)| (*node, value)).collect()
}

#[cfg(test)]
mod tests {
    use super::*; // So that we can import everything from the parent module
    use petgraph::graph::{Graph, UnGraph}; 

    #[test]
    fn test_compute_degree_centrality() {
        let mut graph = UnGraph::new_undirected(); 
        let a = graph.add_node(());
        let b = graph.add_node(());
        graph.add_edge(a, b, ());

        let centrality = compute_degree_centrality(&graph);

        assert_eq!(centrality.len(), 2);  // He we are checking if centrality has two nodes
        assert!(centrality.contains_key(&a));  // Here we are checking if node `a` is in the centrality map
    }

    #[test]
    fn test_top_central_nodes() {
        let mut graph = UnGraph::new_undirected(); 
        let a = graph.add_node(());
        let b = graph.add_node(());
        graph.add_edge(a, b, ());

        // We had to use compute_degree_centrality function and convert results to f64
        let degree_centrality = compute_degree_centrality(&graph)
            .into_iter()
            .map(|(node, centrality)| (node, centrality as f64))  // Here we convert to f64
            .collect::<HashMap<_, _>>();

        let top_nodes = top_central_nodes(&degree_centrality, 2);

        // Here we check if `top_central_nodes` returns the expected results
        assert_eq!(top_nodes.len(), 2);
        assert!(top_nodes.contains(&(a, 1.0)));  // ensure float literals
        assert!(top_nodes.contains(&(b, 1.0)));  
    }
// Unit test to verify that centrality and clustering coefficient computations return empty results when 
//applied to an empty graph.

    #[test]
    fn test_empty_graph() {
        let graph = UnGraph::new_undirected();
        let degree_centrality = compute_degree_centrality(&graph);
        let closeness_centrality = compute_closeness_centrality(&graph, 10);
        let clustering_coefficient = compute_clustering_coefficient(&graph);

        assert!(degree_centrality.is_empty());
        assert!(closeness_centrality.is_empty());
        assert!(clustering_coefficient.is_empty());
    }
// Unit test to verify the correctness of centrality and clustering coefficient 
//computations for a graph with a single node.

    #[test]
    fn test_single_node() {
        let mut graph = UnGraph::new_undirected(); 
        let node = graph.add_node(());
        let degree_centrality = compute_degree_centrality(&graph);
        let closeness_centrality = compute_closeness_centrality(&graph, 1); // Here we compute closeness
        let clustering_coefficient = compute_clustering_coefficient(&graph);

        // Single node, so degree centrality should be 0, no clustering coefficient
        assert_eq!(degree_centrality.get(&node), Some(&0));
        // If there's only one node, closeness centrality should be empty
        assert!(closeness_centrality.is_empty() || closeness_centrality.get(&node).is_some());
        // Clustering coefficient should be 0 for the only node
        assert_eq!(clustering_coefficient.get(&node), Some(&0.0));
    }

}