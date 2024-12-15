mod graph;
mod visualization;
mod report;
mod utils;

use graph::{load_graph, compute_degree_centrality, compute_closeness_centrality, compute_clustering_coefficient, top_central_nodes};
use visualization::{visualize_centrality, visualize_clustering_coefficient};
use report::generate_report;
use utils::normalize_centrality_values;

use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "amazon0302.txt";  
    let graph = load_graph(filename)?;

    let start = Instant::now();
    let degree_centrality = compute_degree_centrality(&graph);
    let closeness_centrality = compute_closeness_centrality(&graph, 150); // The sample size 
    let clustering_coefficient = compute_clustering_coefficient(&graph);

    let normalized_closeness = normalize_centrality_values(&closeness_centrality);
    let top_nodes = top_central_nodes(&normalized_closeness, 10);  // The top 10 nodes

    let duration = start.elapsed();
    println!("Time elapsed for computation: {:?}", duration);

    // The visulizations
    visualize_centrality(&normalized_closeness, "Normalized Closeness Centrality", "closeness.png")?;
    visualize_clustering_coefficient(&clustering_coefficient, "Clustering Coefficients", "clustering.png")?;

    // Here we save the rport 
    generate_report(&degree_centrality, &normalized_closeness, &clustering_coefficient, "report.txt")?;
    
    println!("Top 10 Central Nodes:");
    for (node, centrality) in top_nodes {
        println!("Node: {:?}, Centrality: {}", node.index(), centrality);
    }

    Ok(())
}
