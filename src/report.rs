use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Function that generates a report about the centrality and clustering of the graph
pub fn generate_report(
    degree_centrality: &HashMap<NodeIndex, usize>,
    closeness_centrality: &HashMap<NodeIndex, f64>,
    clustering_coefficient: &HashMap<NodeIndex, f64>,
    output_path: &str,
) -> std::io::Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Degree Centrality:")?;
    for (node, centrality) in degree_centrality {
        writeln!(writer, "Node: {:?}, Degree Centrality: {}", node.index(), centrality)?;
    }

    writeln!(writer, "\nCloseness Centrality:")?;
    for (node, centrality) in closeness_centrality {
        writeln!(writer, "Node: {:?}, Closeness Centrality: {}", node.index(), centrality)?;
    }

    writeln!(writer, "\nClustering Coefficients:")?;
    for (node, coefficient) in clustering_coefficient {
        writeln!(writer, "Node: {:?}, Clustering Coefficient: {}", node.index(), coefficient)?;
    }

    Ok(())
}
